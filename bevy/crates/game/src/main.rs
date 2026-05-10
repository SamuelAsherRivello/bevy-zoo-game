#![cfg_attr(windows, windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_zoo_game::{
    GamePlugin,
    runtime::resources::{
        WindowPlacementStore, create_window_placement_store, valid_window_placement,
    },
};
use bevy_zoo_game_shared::window::{DEFAULT_WINDOW_HEIGHT, DEFAULT_WINDOW_WIDTH};
use std::path::Path;

#[cfg(feature = "desktop-hot-reload")]
use bevy_zoo_game::runtime::resources::record_desktop_hot_reload_patch;
#[cfg(feature = "desktop-hot-reload")]
use dioxus_devtools::{connect_subsecond, subsecond};
#[cfg(feature = "desktop-hot-reload")]
use std::sync::Arc;

fn main() {
    connect_desktop_hot_reload();

    let window_placement_store = create_window_placement_store().ok();
    let saved_window_placement = window_placement_store
        .as_ref()
        .and_then(|store| valid_window_placement(store.current.clone()));
    let window_resolution = saved_window_placement
        .as_ref()
        .map(|placement| WindowResolution::new(placement.window_size.x, placement.window_size.y))
        .unwrap_or_else(|| WindowResolution::new(DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT));
    let window_position = saved_window_placement
        .map(|placement| WindowPosition::At(placement.window_position))
        .unwrap_or(WindowPosition::Centered(MonitorSelection::Primary));

    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(AssetPlugin {
                file_path: game_asset_root(),
                ..default()
            })
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Bevy Zoo Game".to_string(),
                    resolution: window_resolution,
                    position: window_position,
                    ..default()
                }),
                ..default()
            }),
    );

    if let Some(store) = window_placement_store {
        app.insert_resource(store);
    } else {
        app.insert_resource(WindowPlacementStore::default());
    }

    app.add_plugins(GamePlugin).run();
}

fn game_asset_root() -> String {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("assets")
        .to_string_lossy()
        .to_string()
}

#[cfg(feature = "desktop-hot-reload")]
fn connect_desktop_hot_reload() {
    subsecond::register_handler(Arc::new(|| {
        record_desktop_hot_reload_patch();
        info!("Desktop hot reload patch applied");
    }));
    connect_subsecond();
}

#[cfg(not(feature = "desktop-hot-reload"))]
fn connect_desktop_hot_reload() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_asset_root_points_to_crate_assets_directory() {
        let asset_root = Path::new(&game_asset_root()).to_path_buf();

        assert!(asset_root.join("Models").exists());
        assert!(
            asset_root
                .join(bevy_zoo_game::runtime::resources::ZOO_PET_MODEL_PATH)
                .exists()
        );
    }
}
