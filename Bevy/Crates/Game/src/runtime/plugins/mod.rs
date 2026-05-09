use bevy::prelude::*;

use crate::runtime::resources::{
    DebugHudState, GameTicks, PrimaryCameraDefaults, WindowPlacementState, ZooPetDefaults,
    ZooSceneDefaults,
};
use crate::runtime::systems::{
    advance_ticks, load_saved_window_placement, restore_window_placement_to_current_monitors,
    save_window_placement_on_close, scale_debug_hud, setup_debug_hud, setup_game, setup_inspector,
    setup_primary_camera, setup_scene_lighting, setup_zoo_scene, toggle_inspector,
    track_window_placement, track_window_size, update_debug_hud,
};

pub struct CoreGamePlugin;

impl Plugin for CoreGamePlugin {
    fn build(&self, app: &mut App) {
        let camera_defaults = PrimaryCameraDefaults::default();

        app.insert_resource(ClearColor(camera_defaults.clear_color))
            .insert_resource(camera_defaults)
            .init_resource::<Assets<Mesh>>()
            .init_resource::<Assets<StandardMaterial>>()
            .init_resource::<GameTicks>()
            .init_resource::<ZooPetDefaults>()
            .init_resource::<ZooSceneDefaults>()
            .init_resource::<DebugHudState>()
            .init_resource::<WindowPlacementState>()
            .init_resource::<ButtonInput<KeyCode>>()
            .add_systems(
                Startup,
                (
                    load_saved_window_placement,
                    setup_game,
                    setup_primary_camera,
                    setup_scene_lighting,
                    setup_zoo_scene,
                    setup_inspector,
                    setup_debug_hud,
                ),
            )
            .add_systems(
                Update,
                (
                    advance_ticks,
                    restore_window_placement_to_current_monitors,
                    track_window_placement,
                    track_window_size,
                    save_window_placement_on_close.before(bevy::window::close_when_requested),
                    toggle_inspector,
                    update_debug_hud.after(toggle_inspector),
                    scale_debug_hud,
                ),
            );
    }
}
