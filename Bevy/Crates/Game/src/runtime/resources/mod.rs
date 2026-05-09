use bevy::prelude::*;
use bevy_persistent::{error::PersistenceError, prelude::*};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

const WORKSPACE_RELATIVE_FROM_GAME_CRATE: [&str; 3] = ["..", "..", ".."];

pub const PRIMARY_CAMERA_FOV_RADIANS: f32 = std::f32::consts::FRAC_PI_4;
pub const PRIMARY_CAMERA_DISTANCE_FROM_ORIGIN: f32 = 5.0;
pub const PRIMARY_CAMERA_NEAR: f32 = 0.1;
pub const PRIMARY_CAMERA_FAR: f32 = 1000.0;
pub const ZOO_PET_MODEL_PATH: &str =
    "Models/kenney_cube-pets_1.0/Models/GLB format/animal-lion.glb";
pub const ZOO_PET_MODEL_SCALE: f32 = 2.0;
pub const ZOO_FLOOR_MODEL_PATH: &str =
    "Models/kenney_prototype-kit/Models/GLB format/floor-square.glb";
pub const ZOO_TREE_MODEL_PATH: &str = "Models/kenney_platformer-kit/Models/GLB format/tree.glb";

#[derive(Resource, Debug, Default)]
pub struct GameTicks(pub u64);

#[derive(Clone, Debug, Resource)]
pub struct PrimaryCameraDefaults {
    pub position: Vec3,
    pub target: Vec3,
    pub fov_radians: f32,
    pub near: f32,
    pub far: f32,
    pub clear_color: Color,
}

impl Default for PrimaryCameraDefaults {
    fn default() -> Self {
        Self {
            position: Vec3::new(0.0, 2.5, PRIMARY_CAMERA_DISTANCE_FROM_ORIGIN),
            target: Vec3::ZERO,
            fov_radians: PRIMARY_CAMERA_FOV_RADIANS,
            near: PRIMARY_CAMERA_NEAR,
            far: PRIMARY_CAMERA_FAR,
            clear_color: Color::srgb(0.08, 0.08, 0.08),
        }
    }
}

impl PrimaryCameraDefaults {
    pub fn transform(&self) -> Transform {
        Transform::from_translation(self.position).looking_at(self.target, Vec3::Y)
    }
}

#[derive(Clone, Debug, Resource)]
pub struct ZooPetDefaults {
    pub model_path: &'static str,
    pub transform: Transform,
}

impl Default for ZooPetDefaults {
    fn default() -> Self {
        Self {
            model_path: ZOO_PET_MODEL_PATH,
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0))
                .with_scale(Vec3::splat(ZOO_PET_MODEL_SCALE)),
        }
    }
}

#[derive(Clone, Debug, Resource)]
pub struct ZooSceneDefaults {
    pub floor_model_path: &'static str,
    pub floor_transform: Transform,
    pub tree_model_path: &'static str,
    pub tree_transform: Transform,
}

impl Default for ZooSceneDefaults {
    fn default() -> Self {
        Self {
            floor_model_path: ZOO_FLOOR_MODEL_PATH,
            floor_transform: Transform::from_translation(Vec3::new(0.0, -0.05, 0.0))
                .with_scale(Vec3::splat(4.0)),
            tree_model_path: ZOO_TREE_MODEL_PATH,
            tree_transform: Transform::from_translation(Vec3::new(1.2, 0.0, -0.8))
                .with_scale(Vec3::splat(1.4)),
        }
    }
}

#[derive(Resource, Debug, Default)]
pub struct DebugHudState {
    pub is_fps_visible: bool,
    pub fps_accumulated_seconds: f32,
    pub fps_accumulated_frames: u32,
    pub fps_display_value: f32,
}

#[derive(Resource, Debug, Default)]
pub struct WindowPlacementState {
    pub current: Option<WindowPlacement>,
    pub restored: bool,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Resource, Serialize)]
pub struct WindowPlacementStore {
    pub current: Option<WindowPlacement>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct WindowPlacement {
    pub window_position: IVec2,
    pub window_size: UVec2,
    pub monitor_name: Option<String>,
    pub monitor_position: IVec2,
    pub monitor_size: UVec2,
    pub relative_position: IVec2,
}

pub fn window_placement_path() -> PathBuf {
    workspace_root_path()
        .join("data")
        .join("local_storage")
        .join("window-placement.json")
}

pub fn create_window_placement_store() -> Result<Persistent<WindowPlacementStore>, PersistenceError>
{
    Persistent::<WindowPlacementStore>::builder()
        .name("window placement")
        .format(StorageFormat::JsonPretty)
        .path(window_placement_path())
        .default(WindowPlacementStore::default())
        .revertible(true)
        .revert_to_default_on_deserialization_errors(true)
        .build()
}

pub fn load_window_placement() -> Option<WindowPlacement> {
    valid_window_placement(create_window_placement_store().ok()?.current.clone())
}

pub fn valid_window_placement(placement: Option<WindowPlacement>) -> Option<WindowPlacement> {
    placement.filter(is_valid_window_placement)
}

fn workspace_root_path() -> PathBuf {
    let mut path = Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf();
    for component in WORKSPACE_RELATIVE_FROM_GAME_CRATE {
        path.push(component);
    }
    path
}

fn is_valid_window_placement(placement: &WindowPlacement) -> bool {
    placement.window_size.x > 0 && placement.window_size.y > 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn window_placement_serializes_position_size_and_screen_identity() {
        let placement = WindowPlacement {
            window_position: IVec2::new(100, 200),
            window_size: UVec2::new(800, 600),
            monitor_name: Some("Display 1".to_string()),
            monitor_position: IVec2::ZERO,
            monitor_size: UVec2::new(1920, 1080),
            relative_position: IVec2::new(100, 200),
        };

        let serialized = serde_json::to_string(&placement).unwrap();
        let deserialized: WindowPlacement = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized, placement);
    }

    #[test]
    fn window_placement_requires_positive_size() {
        let placement = WindowPlacement {
            window_position: IVec2::new(100, 200),
            window_size: UVec2::ZERO,
            monitor_name: None,
            monitor_position: IVec2::ZERO,
            monitor_size: UVec2::new(1920, 1080),
            relative_position: IVec2::new(100, 200),
        };

        assert_eq!(valid_window_placement(Some(placement)), None);
    }

    #[test]
    fn window_placement_uses_workspace_local_storage() {
        let path = window_placement_path();
        assert!(
            path.ends_with(
                Path::new("data")
                    .join("local_storage")
                    .join("window-placement.json")
            )
        );
        assert!(!path.components().any(|component| {
            component.as_os_str() == "game" && path.to_string_lossy().contains("game\\data")
        }));
    }

    #[test]
    fn zoo_pet_defaults_use_validated_cube_pet_glb() {
        let defaults = ZooPetDefaults::default();

        assert!(
            defaults
                .model_path
                .starts_with("Models/kenney_cube-pets_1.0/")
        );
        assert!(defaults.model_path.ends_with(".glb"));
        assert_eq!(defaults.transform.scale, Vec3::splat(ZOO_PET_MODEL_SCALE));
    }
}
