use bevy::{asset::RenderAssetUsages, mesh::Indices, prelude::*};
use serde::Deserialize;
use std::collections::HashMap;

pub fn mesh_from_glb(source: &[u8]) -> Result<Mesh, String> {
    let (document, binary) = parse_glb(source)?;
    let primitive = document
        .meshes
        .first()
        .and_then(|mesh| mesh.primitives.first())
        .ok_or_else(|| "GLB contains no mesh primitives".to_string())?;
    let position_accessor = primitive
        .attributes
        .get("POSITION")
        .ok_or_else(|| "GLB primitive has no positions".to_string())?;
    let positions = read_vec3_accessor(&document, binary, *position_accessor)?;
    let normals = primitive
        .attributes
        .get("NORMAL")
        .map(|accessor| read_vec3_accessor(&document, binary, *accessor))
        .transpose()?
        .unwrap_or_else(|| vec![[0.0, 1.0, 0.0]; positions.len()]);
    let uvs = primitive
        .attributes
        .get("TEXCOORD_0")
        .map(|accessor| read_vec2_accessor(&document, binary, *accessor))
        .transpose()?
        .unwrap_or_else(|| vec![[0.0, 0.0]; positions.len()]);
    let indices = primitive
        .indices
        .map(|accessor| read_index_accessor(&document, binary, accessor))
        .transpose()?
        .unwrap_or_else(|| (0..positions.len() as u32).collect());

    if positions.len() != normals.len() || positions.len() != uvs.len() {
        return Err("GLB vertex attributes have mismatched counts".to_string());
    }

    let mut mesh = Mesh::new(
        bevy::mesh::PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(Indices::U32(indices));
    Ok(mesh)
}

fn parse_glb(source: &[u8]) -> Result<(GltfDocument, &[u8]), String> {
    if source.len() < 20 {
        return Err("GLB is too small".to_string());
    }
    if read_u32(source, 0)? != 0x4654_6c67 {
        return Err("GLB magic header is invalid".to_string());
    }

    let mut offset = 12;
    let mut json = None;
    let mut binary = None;
    while offset + 8 <= source.len() {
        let chunk_length = read_u32(source, offset)? as usize;
        let chunk_type = read_u32(source, offset + 4)?;
        let chunk_start = offset + 8;
        let chunk_end = chunk_start + chunk_length;
        if chunk_end > source.len() {
            return Err("GLB chunk extends past file length".to_string());
        }

        match chunk_type {
            0x4e4f_534a => json = Some(&source[chunk_start..chunk_end]),
            0x004e_4942 => binary = Some(&source[chunk_start..chunk_end]),
            _ => {}
        }
        offset = chunk_end;
    }

    let json = json.ok_or_else(|| "GLB has no JSON chunk".to_string())?;
    let binary = binary.ok_or_else(|| "GLB has no binary chunk".to_string())?;
    let document = serde_json::from_slice(json).map_err(|error| error.to_string())?;
    Ok((document, binary))
}

fn read_vec3_accessor(
    document: &GltfDocument,
    binary: &[u8],
    accessor_index: usize,
) -> Result<Vec<[f32; 3]>, String> {
    let values = read_f32_accessor(document, binary, accessor_index, "VEC3")?;
    Ok(values
        .chunks_exact(3)
        .map(|chunk| [chunk[0], chunk[1], chunk[2]])
        .collect())
}

fn read_vec2_accessor(
    document: &GltfDocument,
    binary: &[u8],
    accessor_index: usize,
) -> Result<Vec<[f32; 2]>, String> {
    let values = read_f32_accessor(document, binary, accessor_index, "VEC2")?;
    Ok(values
        .chunks_exact(2)
        .map(|chunk| [chunk[0], chunk[1]])
        .collect())
}

fn read_f32_accessor(
    document: &GltfDocument,
    binary: &[u8],
    accessor_index: usize,
    expected_type: &str,
) -> Result<Vec<f32>, String> {
    let accessor = document.accessor(accessor_index)?;
    if accessor.component_type != 5126 || accessor.kind != expected_type {
        return Err(format!(
            "accessor {accessor_index} is not {expected_type} f32"
        ));
    }

    let component_count = component_count(&accessor.kind)?;
    let view = document.buffer_view(accessor.buffer_view)?;
    let stride = view.byte_stride.unwrap_or(component_count * 4);
    let start = view.byte_offset + accessor.byte_offset;
    let mut values = Vec::with_capacity(accessor.count * component_count);

    for element in 0..accessor.count {
        let element_start = start + element * stride;
        for component in 0..component_count {
            values.push(read_f32(binary, element_start + component * 4)?);
        }
    }

    Ok(values)
}

fn read_index_accessor(
    document: &GltfDocument,
    binary: &[u8],
    accessor_index: usize,
) -> Result<Vec<u32>, String> {
    let accessor = document.accessor(accessor_index)?;
    let view = document.buffer_view(accessor.buffer_view)?;
    let component_size = match accessor.component_type {
        5121 => 1,
        5123 => 2,
        5125 => 4,
        _ => {
            return Err(format!(
                "unsupported index component type {}",
                accessor.component_type
            ));
        }
    };
    let stride = view.byte_stride.unwrap_or(component_size);
    let start = view.byte_offset + accessor.byte_offset;
    let mut values = Vec::with_capacity(accessor.count);

    for element in 0..accessor.count {
        let element_start = start + element * stride;
        values.push(match accessor.component_type {
            5121 => read_u8(binary, element_start)? as u32,
            5123 => read_u16(binary, element_start)? as u32,
            5125 => read_u32(binary, element_start)?,
            _ => unreachable!(),
        });
    }

    Ok(values)
}

fn component_count(kind: &str) -> Result<usize, String> {
    match kind {
        "SCALAR" => Ok(1),
        "VEC2" => Ok(2),
        "VEC3" => Ok(3),
        "VEC4" => Ok(4),
        _ => Err(format!("unsupported accessor type {kind}")),
    }
}

fn read_u8(source: &[u8], offset: usize) -> Result<u8, String> {
    source
        .get(offset)
        .copied()
        .ok_or_else(|| "read past binary chunk".to_string())
}

fn read_u16(source: &[u8], offset: usize) -> Result<u16, String> {
    let bytes = source
        .get(offset..offset + 2)
        .ok_or_else(|| "read past binary chunk".to_string())?;
    Ok(u16::from_le_bytes([bytes[0], bytes[1]]))
}

fn read_u32(source: &[u8], offset: usize) -> Result<u32, String> {
    let bytes = source
        .get(offset..offset + 4)
        .ok_or_else(|| "read past binary chunk".to_string())?;
    Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}

fn read_f32(source: &[u8], offset: usize) -> Result<f32, String> {
    let bytes = source
        .get(offset..offset + 4)
        .ok_or_else(|| "read past binary chunk".to_string())?;
    Ok(f32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}

#[derive(Deserialize)]
struct GltfDocument {
    accessors: Vec<GltfAccessor>,
    #[serde(rename = "bufferViews")]
    buffer_views: Vec<GltfBufferView>,
    meshes: Vec<GltfMesh>,
}

impl GltfDocument {
    fn accessor(&self, index: usize) -> Result<&GltfAccessor, String> {
        self.accessors
            .get(index)
            .ok_or_else(|| format!("missing accessor {index}"))
    }

    fn buffer_view(&self, index: usize) -> Result<&GltfBufferView, String> {
        self.buffer_views
            .get(index)
            .ok_or_else(|| format!("missing buffer view {index}"))
    }
}

#[derive(Deserialize)]
struct GltfAccessor {
    #[serde(rename = "bufferView")]
    buffer_view: usize,
    #[serde(default, rename = "byteOffset")]
    byte_offset: usize,
    #[serde(rename = "componentType")]
    component_type: u32,
    count: usize,
    #[serde(rename = "type")]
    kind: String,
}

#[derive(Deserialize)]
struct GltfBufferView {
    #[serde(default, rename = "byteOffset")]
    byte_offset: usize,
    #[serde(rename = "byteStride")]
    byte_stride: Option<usize>,
}

#[derive(Deserialize)]
struct GltfMesh {
    primitives: Vec<GltfPrimitive>,
}

#[derive(Deserialize)]
struct GltfPrimitive {
    attributes: HashMap<String, usize>,
    indices: Option<usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    const ZOO_PET_GLB: &[u8] = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/Assets/Models/kenney_cube-pets_1.0/Models/GLB format/animal-lion.glb"
    ));

    #[test]
    fn cube_pet_glb_builds_renderable_mesh() {
        let mesh = mesh_from_glb(ZOO_PET_GLB).unwrap();

        assert_eq!(
            mesh.primitive_topology(),
            bevy::mesh::PrimitiveTopology::TriangleList
        );
        assert!(mesh.count_vertices() > 0);
        assert!(mesh.attribute(Mesh::ATTRIBUTE_POSITION).is_some());
        assert!(mesh.attribute(Mesh::ATTRIBUTE_NORMAL).is_some());
        assert!(mesh.attribute(Mesh::ATTRIBUTE_UV_0).is_some());
    }
}
