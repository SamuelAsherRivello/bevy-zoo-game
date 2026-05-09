use bevy::prelude::*;
use bevy_persistent::{error::PersistenceError, prelude::*};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
#[cfg(feature = "desktop-hot-reload")]
use std::sync::atomic::{AtomicU64, Ordering};

const WORKSPACE_RELATIVE_FROM_GAME_CRATE: [&str; 3] = ["..", "..", ".."];
#[cfg(feature = "desktop-hot-reload")]
static DESKTOP_HOT_RELOAD_PATCH_COUNT: AtomicU64 = AtomicU64::new(0);

pub const PRIMARY_CAMERA_FOV_RADIANS: f32 = std::f32::consts::FRAC_PI_4;
pub const PRIMARY_CAMERA_DISTANCE_FROM_ORIGIN: f32 = 7.0;
pub const PRIMARY_CAMERA_NEAR: f32 = 0.1;
pub const PRIMARY_CAMERA_FAR: f32 = 1000.0;
pub const ZOO_PET_MODEL_PATH: &str =
    "Models/kenney_cube-pets_1.0/Models/GLB format/animal-polar.glb";
pub const ZOO_PET_MODEL_SCALE: f32 = 0.5;
pub const ZOO_FLOOR_MODEL_PATH: &str =
    "Models/kenney_platformer-kit/Models/GLB format/block-grass-overhang-large.glb";
pub const ZOO_TREE_MODEL_PATH: &str =
    "Models/kenney_graveyard-kit_5.0/Models/GLB format/pine-crooked.glb";
pub const MODEL_BROWSER_GRID_COLUMNS: usize = 13;
pub const MODEL_BROWSER_GRID_ROWS: usize = 12;
pub const MODEL_BROWSER_MODEL_COUNT: usize = MODEL_BROWSER_GRID_COLUMNS * MODEL_BROWSER_GRID_ROWS;
pub const MODEL_BROWSER_GRID_SPACING: f32 = 1.05;
pub const MODEL_BROWSER_GRID_SCALE: f32 = 0.32;
pub const MODEL_BROWSER_SHOWCASE_SCALE: f32 = 2.688;
pub const MODEL_BROWSER_PICK_RADIUS: f32 = 0.52;
pub const MODEL_BROWSER_CAMERA_Z: f32 = 22.0;
pub const MODEL_BROWSER_SHOWCASE_Z: f32 = 13.0;
pub const MODEL_BROWSER_PAGE_ORDER: [ModelBrowserPageFolder; 4] = [
    ModelBrowserPageFolder {
        title: "Pets",
        folder: "kenney_cube-pets_1.0",
    },
    ModelBrowserPageFolder {
        title: "Graveyard",
        folder: "kenney_graveyard-kit_5.0",
    },
    ModelBrowserPageFolder {
        title: "Platformer",
        folder: "kenney_platformer-kit",
    },
    ModelBrowserPageFolder {
        title: "Prototype",
        folder: "kenney_prototype-kit",
    },
];

pub const MODEL_BROWSER_ANIMAL_PATHS: [&str; 24] = [
    "Models/kenney_cube-pets_1.0/Models/GLB format/animal-beaver.glb",
    "Models/kenney_cube-pets_1.0/Models/GLB format/animal-bee.glb",
    "Models/kenney_cube-pets_1.0/Models/GLB format/animal-bunny.glb",
    "Models/kenney_cube-pets_1.0/Models/GLB format/animal-cat.glb",
    "Models/kenney_cube-pets_1.0/Models/GLB format/animal-caterpillar.glb",
    "Models/kenney_cube-pets_1.0/Models/GLB format/animal-chick.glb",
    "Models/kenney_cube-pets_1.0/Models/GLB format/animal-cow.glb",
    "Models/kenney_cube-pets_1.0/Models/GLB format/animal-crab.glb",
    "Models/kenney_cube-pets_1.0/Models/GLB format/animal-deer.glb",
    "Models/kenney_cube-pets_1.0/Models/GLB format/animal-dog.glb",
    "Models/kenney_cube-pets_1.0/Models/GLB format/animal-elephant.glb",
    "Models/kenney_cube-pets_1.0/Models/GLB format/animal-fish.glb",
    "Models/kenney_cube-pets_1.0/Models/GLB format/animal-fox.glb",
    "Models/kenney_cube-pets_1.0/Models/GLB format/animal-giraffe.glb",
    "Models/kenney_cube-pets_1.0/Models/GLB format/animal-hog.glb",
    "Models/kenney_cube-pets_1.0/Models/GLB format/animal-koala.glb",
    "Models/kenney_cube-pets_1.0/Models/GLB format/animal-lion.glb",
    "Models/kenney_cube-pets_1.0/Models/GLB format/animal-monkey.glb",
    "Models/kenney_cube-pets_1.0/Models/GLB format/animal-panda.glb",
    "Models/kenney_cube-pets_1.0/Models/GLB format/animal-parrot.glb",
    "Models/kenney_cube-pets_1.0/Models/GLB format/animal-penguin.glb",
    "Models/kenney_cube-pets_1.0/Models/GLB format/animal-pig.glb",
    "Models/kenney_cube-pets_1.0/Models/GLB format/animal-polar.glb",
    "Models/kenney_cube-pets_1.0/Models/GLB format/animal-tiger.glb",
];

#[derive(Resource, Debug, Default)]
pub struct GameTicks(pub u64);

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Resource)]
pub enum ActiveScene {
    #[default]
    GameScene,
    ModelBrowser,
}

#[derive(Debug, Default, Resource)]
pub struct ModelBrowserSelection {
    pub selected: Option<Entity>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ModelBrowserPageFolder {
    pub title: &'static str,
    pub folder: &'static str,
}

#[derive(Debug, Default, Resource)]
pub struct ModelBrowserPage {
    pub current: usize,
}

impl ModelBrowserPage {
    pub fn current_folder(&self) -> ModelBrowserPageFolder {
        MODEL_BROWSER_PAGE_ORDER[self.current % MODEL_BROWSER_PAGE_ORDER.len()]
    }

    pub fn next(&mut self) {
        self.current = (self.current + 1) % MODEL_BROWSER_PAGE_ORDER.len();
    }

    pub fn back(&mut self) {
        self.current =
            (self.current + MODEL_BROWSER_PAGE_ORDER.len() - 1) % MODEL_BROWSER_PAGE_ORDER.len();
    }
}

pub fn model_browser_page_paths(folder: ModelBrowserPageFolder) -> Vec<String> {
    let model_folder = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("Assets")
        .join("Models")
        .join(folder.folder)
        .join("Models")
        .join("GLB format");

    let Ok(entries) = std::fs::read_dir(model_folder) else {
        return Vec::new();
    };

    let mut paths = entries
        .filter_map(Result::ok)
        .filter_map(|entry| {
            let path = entry.path();
            let extension = path.extension()?.to_str()?;
            if !extension.eq_ignore_ascii_case("glb") {
                return None;
            }
            let file_name = path.file_name()?.to_str()?;
            Some(format!(
                "Models/{}/Models/GLB format/{}",
                folder.folder, file_name
            ))
        })
        .collect::<Vec<_>>();

    paths.sort();
    paths
}

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
            position: Vec3::new(0.0, 3.2, PRIMARY_CAMERA_DISTANCE_FROM_ORIGIN),
            target: Vec3::new(0.0, 0.8, 0.0),
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
            transform: Transform::from_translation(Vec3::new(0.0, 0.9, 0.0))
                .with_rotation(Quat::from_euler(EulerRot::XYZ, 0.0, 0.3, 0.0))
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
            floor_transform: Transform::from_translation(Vec3::new(0.0, -1.3, 0.0))
                .with_scale(Vec3::new(2.0, 1.5, 2.0)),
            tree_model_path: ZOO_TREE_MODEL_PATH,
            tree_transform: Transform::from_translation(Vec3::new(1.45, 0.1, -0.35))
                .with_scale(Vec3::splat(1.2)),
        }
    }
}

#[derive(Resource, Debug, Default)]
pub struct DebugHudState {
    pub is_fps_visible: bool,
    pub is_inspector_visible: bool,
    pub is_hot_reload_autorestart_enabled: bool,
    pub fps_accumulated_seconds: f32,
    pub fps_accumulated_frames: u32,
    pub fps_display_value: f32,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Resource, Serialize)]
pub struct DebugHudInputStore {
    pub is_fps_visible: bool,
    pub is_inspector_visible: bool,
    pub is_hot_reload_autorestart_enabled: bool,
}

impl DebugHudInputStore {
    pub fn from_state(state: &DebugHudState) -> Self {
        Self {
            is_fps_visible: state.is_fps_visible,
            is_inspector_visible: state.is_inspector_visible,
            is_hot_reload_autorestart_enabled: state.is_hot_reload_autorestart_enabled,
        }
    }

    pub fn apply_to_state(&self, state: &mut DebugHudState) {
        state.is_fps_visible = self.is_fps_visible;
        state.is_inspector_visible = self.is_inspector_visible;
        state.is_hot_reload_autorestart_enabled = self.is_hot_reload_autorestart_enabled;
    }
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

pub fn debug_hud_input_path() -> PathBuf {
    workspace_root_path()
        .join("data")
        .join("local_storage")
        .join("debug-hud-input.json")
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

pub fn create_debug_hud_input_store() -> Result<Persistent<DebugHudInputStore>, PersistenceError> {
    Persistent::<DebugHudInputStore>::builder()
        .name("debug hud input")
        .format(StorageFormat::JsonPretty)
        .path(debug_hud_input_path())
        .default(DebugHudInputStore::default())
        .revertible(true)
        .revert_to_default_on_deserialization_errors(true)
        .build()
}

pub fn load_window_placement() -> Option<WindowPlacement> {
    valid_window_placement(create_window_placement_store().ok()?.current.clone())
}

#[cfg(feature = "desktop-hot-reload")]
pub fn record_desktop_hot_reload_patch() {
    DESKTOP_HOT_RELOAD_PATCH_COUNT.fetch_add(1, Ordering::Relaxed);
}

#[cfg(feature = "desktop-hot-reload")]
pub fn desktop_hot_reload_patch_count() -> u64 {
    DESKTOP_HOT_RELOAD_PATCH_COUNT.load(Ordering::Relaxed)
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
    fn debug_hud_input_uses_workspace_local_storage() {
        let path = debug_hud_input_path();
        assert!(
            path.ends_with(
                Path::new("data")
                    .join("local_storage")
                    .join("debug-hud-input.json")
            )
        );
    }

    #[test]
    fn debug_hud_input_defaults_all_toggles_off() {
        let store = DebugHudInputStore::default();

        assert!(!store.is_fps_visible);
        assert!(!store.is_inspector_visible);
        assert!(!store.is_hot_reload_autorestart_enabled);
    }

    #[test]
    fn zoo_pet_defaults_use_validated_cube_pet_glb() {
        let defaults = ZooPetDefaults::default();

        assert!(
            defaults
                .model_path
                .starts_with("Models/kenney_cube-pets_1.0/")
        );
        assert!(defaults.model_path.ends_with("animal-polar.glb"));
        assert_eq!(defaults.transform.scale, Vec3::splat(ZOO_PET_MODEL_SCALE));
    }

    #[test]
    fn zoo_scene_defaults_use_crooked_pine_tree_glb() {
        let defaults = ZooSceneDefaults::default();

        assert!(
            defaults
                .tree_model_path
                .starts_with("Models/kenney_graveyard-kit_5.0/")
        );
        assert!(defaults.tree_model_path.ends_with("pine-crooked.glb"));
    }

    #[test]
    fn model_browser_pages_follow_requested_folder_order() {
        let titles = MODEL_BROWSER_PAGE_ORDER
            .iter()
            .map(|folder| folder.title)
            .collect::<Vec<_>>();

        assert_eq!(titles, vec!["Pets", "Graveyard", "Platformer", "Prototype"]);
    }

    #[test]
    fn model_browser_page_paths_read_glbs_from_page_folder() {
        let paths = model_browser_page_paths(MODEL_BROWSER_PAGE_ORDER[0]);

        assert_eq!(paths.len(), 24);
        assert_eq!(
            paths.first().map(String::as_str),
            Some("Models/kenney_cube-pets_1.0/Models/GLB format/animal-beaver.glb")
        );
        assert!(paths.iter().all(|path| path.ends_with(".glb")));
    }
}
