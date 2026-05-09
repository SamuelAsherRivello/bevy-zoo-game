use bevy::{light::DirectionalLightShadowMap, prelude::*};

use crate::runtime::resources::{
    ActiveScene, DebugHudState, GameTicks, ModelBrowserPage, ModelBrowserSelection,
    PrimaryCameraDefaults, WindowPlacementState, ZooPetDefaults, ZooSceneDefaults,
    create_debug_hud_input_store,
};
use crate::runtime::systems::{
    advance_ticks, hot_reload_auto_restart_zoo_scene, load_saved_debug_hud_input,
    load_saved_window_placement, model_browser_click_selection, model_browser_page_navigation,
    restart_zoo_scene, restore_window_placement_to_current_monitors, rotation_system,
    save_window_placement_on_close, scale_debug_hud, setup_app_scene, setup_game, setup_inspector,
    setup_zoo_scene, toggle_debug_hud_inputs, toggle_scene_browser, track_window_placement,
    track_window_size, update_debug_hud, update_model_browser_metadata,
};

pub struct CoreGamePlugin;

impl Plugin for CoreGamePlugin {
    fn build(&self, app: &mut App) {
        let camera_defaults = PrimaryCameraDefaults::default();

        app.insert_resource(ClearColor(camera_defaults.clear_color))
            .insert_resource(DirectionalLightShadowMap { size: 4096 })
            .insert_resource(camera_defaults)
            .init_resource::<Assets<Mesh>>()
            .init_resource::<Assets<StandardMaterial>>()
            .init_resource::<Assets<Image>>()
            .init_resource::<GameTicks>()
            .init_resource::<ActiveScene>()
            .init_resource::<ModelBrowserSelection>()
            .init_resource::<ModelBrowserPage>()
            .init_resource::<ZooPetDefaults>()
            .init_resource::<ZooSceneDefaults>()
            .init_resource::<DebugHudState>()
            .init_resource::<WindowPlacementState>()
            .init_resource::<ButtonInput<KeyCode>>()
            .add_systems(
                Startup,
                (
                    load_saved_window_placement,
                    load_saved_debug_hud_input,
                    setup_game,
                    setup_app_scene,
                    setup_zoo_scene,
                    setup_inspector,
                )
                    .chain(),
            )
            .add_systems(
                Update,
                (
                    advance_ticks,
                    rotation_system,
                    restore_window_placement_to_current_monitors,
                    track_window_placement,
                    track_window_size,
                    save_window_placement_on_close.before(bevy::window::close_when_requested),
                    toggle_debug_hud_inputs,
                    toggle_scene_browser,
                    model_browser_page_navigation,
                    model_browser_click_selection,
                    update_model_browser_metadata
                        .after(model_browser_page_navigation)
                        .after(model_browser_click_selection),
                    restart_zoo_scene,
                    hot_reload_auto_restart_zoo_scene,
                    update_debug_hud.after(toggle_debug_hud_inputs),
                    scale_debug_hud,
                ),
            );

        if let Ok(store) = create_debug_hud_input_store() {
            app.insert_resource(store);
        }
    }
}
