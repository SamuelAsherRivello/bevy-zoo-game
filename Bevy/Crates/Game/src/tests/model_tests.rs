use std::{
    fs,
    path::{Path, PathBuf},
    time::Duration,
};

use bevy::{
    animation::AnimationClip,
    asset::{AssetPlugin, LoadState},
    gltf::{Gltf, GltfPlugin},
    image::Image,
    mesh::skinning::SkinnedMeshInverseBindposes,
    pbr::StandardMaterial,
    prelude::*,
    scene::Scene,
};

const ASSET_ROOT_RELATIVE: &str = "Assets";
const MODEL_ROOT_RELATIVE: &str = "Assets/Models";
const OUTPUT_PATH_RELATIVE: &str = "src/tests/model-test-output.md";

#[test]
fn curated_glb_models_load() {
    let asset_root = manifest_path(ASSET_ROOT_RELATIVE);
    let model_root = manifest_path(MODEL_ROOT_RELATIVE);
    let output_path = manifest_path(OUTPUT_PATH_RELATIVE);
    let models = discover_glb_models(&asset_root, &model_root);
    assert!(
        !models.is_empty(),
        "no GLB models found under {}",
        model_root.display()
    );

    let mut app = App::new();
    app.add_plugins((
        MinimalPlugins,
        AssetPlugin {
            file_path: asset_root.to_string_lossy().to_string(),
            ..default()
        },
        GltfPlugin::default(),
    ));
    app.init_asset::<AnimationClip>()
        .init_asset::<Image>()
        .init_asset::<Mesh>()
        .init_asset::<Scene>()
        .init_asset::<SkinnedMeshInverseBindposes>()
        .init_asset::<StandardMaterial>();
    app.finish();
    app.cleanup();

    let results = load_models(&mut app, &models);
    write_report(&results, &output_path);

    let failed_models = results
        .iter()
        .filter(|result| !result.loaded)
        .map(|result| {
            format!(
                "{} ({})",
                result.asset_path,
                result.comment.as_deref().unwrap_or("load failed")
            )
        })
        .collect::<Vec<_>>();

    assert!(
        failed_models.is_empty(),
        "GLB models failed to load:\n{}",
        failed_models.join("\n")
    );
}

fn discover_glb_models(asset_root: &Path, root: &Path) -> Vec<ModelCandidate> {
    let mut models = Vec::new();
    collect_glb_models(asset_root, root, &mut models);
    models.sort_by(|left, right| left.asset_path.cmp(&right.asset_path));
    models
}

fn collect_glb_models(asset_root: &Path, path: &Path, models: &mut Vec<ModelCandidate>) {
    let Ok(entries) = fs::read_dir(path) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_glb_models(asset_root, &path, models);
            continue;
        }

        if path.extension().and_then(|extension| extension.to_str()) != Some("glb") {
            continue;
        }

        let Some(asset_path) = to_asset_path(asset_root, &path) else {
            continue;
        };
        let Some(source_folder) = path.parent().map(Path::to_path_buf) else {
            continue;
        };

        let file_name = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or_default()
            .to_string();
        let asset_type = model_type(&asset_path);

        models.push(ModelCandidate {
            file_name,
            asset_path,
            source_folder,
            asset_type,
        });
    }
}

fn load_models(app: &mut App, models: &[ModelCandidate]) -> Vec<ModelTestResult> {
    let asset_server = app.world().resource::<AssetServer>().clone();
    let handles = models
        .iter()
        .map(|model| asset_server.load::<Gltf>(model.asset_path.clone()))
        .collect::<Vec<_>>();

    for _ in 0..300 {
        app.update();

        if handles.iter().all(|handle| {
            matches!(
                asset_server.get_load_state(handle.id()),
                Some(LoadState::Loaded | LoadState::Failed(_))
            )
        }) {
            break;
        }

        std::thread::sleep(Duration::from_millis(10));
    }

    let gltf_assets = app.world().resource::<Assets<Gltf>>();

    models
        .iter()
        .zip(handles)
        .map(|(model, handle)| {
            let load_state = asset_server.get_load_state(handle.id());
            let loaded_asset = gltf_assets.get(&handle);
            let loaded = matches!(load_state, Some(LoadState::Loaded))
                && loaded_asset.is_some_and(|gltf| !gltf.scenes.is_empty());
            let load_comment = match (load_state, loaded_asset) {
                (Some(LoadState::Loaded), Some(gltf)) if gltf.scenes.is_empty() => {
                    Some("Loaded, but no scenes were found.".to_string())
                }
                (Some(LoadState::Loaded), _) => None,
                (Some(LoadState::Failed(error)), _) => Some(format!("Load failed: {error}")),
                (Some(other), _) => Some(format!("Load did not finish: {other:?}")),
                (None, _) => Some("Load state was unavailable.".to_string()),
            };

            ModelTestResult {
                file_name: model.file_name.clone(),
                asset_path: model.asset_path.clone(),
                source_folder: model.source_folder.clone(),
                keep: loaded,
                loaded,
                asset_type: model.asset_type.clone(),
                comment: load_comment,
            }
        })
        .collect()
}

fn write_report(results: &[ModelTestResult], output_path: &Path) {
    let mut output = String::from(
        "# Model Test Output\n\n\
        Generated by `cargo test -p bevy-zoo-game curated_glb_models_load`.\n\n\
        | Model | Keep | Loads | Type | Comment |\n\
        | ----- | ---- | ----- | ---- | ------- |\n",
    );

    for result in results {
        let model_link = format!(
            "[{}]({})",
            result.file_name,
            markdown_folder_link(&result.source_folder, output_path)
        );
        let keep = if result.keep { "✅" } else { "❌" };
        let loads = if result.loaded { "✅" } else { "❌" };
        let asset_type = result.asset_type.as_deref().unwrap_or("");
        let comment = result.comment.as_deref().unwrap_or("");

        output.push_str(&format!(
            "| {} | {} | {} | {} | {} |\n",
            model_link,
            keep,
            loads,
            escape_table_cell(asset_type),
            escape_table_cell(comment)
        ));
    }

    fs::write(output_path, output).expect("model test report should be writable");
}

fn model_type(asset_path: &str) -> Option<String> {
    let mut parts = asset_path.split('/');
    let package = match (parts.next(), parts.next()) {
        (Some(root), Some(package)) => format!("{root}/{package}"),
        _ => String::new(),
    };

    match package.as_str() {
        "Models/kenney_cube-pets_1.0" => Some("zoo".to_string()),
        "Models/kenney_platformer-kit" => Some("platformer".to_string()),
        "Models/kenney_graveyard-kit_5.0" => Some("graveyard".to_string()),
        "Models/kenney_prototype-kit" => Some("prototype".to_string()),
        _ => None,
    }
}

fn to_asset_path(asset_root: &Path, path: &Path) -> Option<String> {
    path.strip_prefix(asset_root)
        .ok()
        .map(|path| path.to_string_lossy().replace('\\', "/"))
}

fn markdown_folder_link(source_folder: &Path, output_path: &Path) -> String {
    let output_parent = output_path.parent().unwrap_or(Path::new("."));
    let relative_path = source_folder
        .strip_prefix(output_parent)
        .map(Path::to_path_buf)
        .unwrap_or_else(|_| {
            Path::new("../../Assets").join(
                source_folder
                    .strip_prefix(manifest_path(ASSET_ROOT_RELATIVE))
                    .unwrap_or(source_folder),
            )
        });
    relative_path
        .to_string_lossy()
        .replace('\\', "/")
        .replace(' ', "%20")
}

fn manifest_path(relative_path: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(relative_path)
}

fn escape_table_cell(value: &str) -> String {
    value.replace('|', "\\|")
}

struct ModelCandidate {
    file_name: String,
    asset_path: String,
    source_folder: PathBuf,
    asset_type: Option<String>,
}

struct ModelTestResult {
    file_name: String,
    asset_path: String,
    source_folder: PathBuf,
    keep: bool,
    loaded: bool,
    asset_type: Option<String>,
    comment: Option<String>,
}
