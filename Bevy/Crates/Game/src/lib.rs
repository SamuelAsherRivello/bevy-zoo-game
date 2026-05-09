use bevy::prelude::*;
use bevy_inspector_egui::{
    DefaultInspectorConfigPlugin,
    bevy_egui::{EguiPlugin, EguiPrimaryContextPass},
};

pub mod runtime;

use runtime::plugins::CoreGamePlugin;
use runtime::systems::inspector_ui;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CoreGamePlugin)
            .add_plugins(EguiPlugin::default())
            .add_plugins(DefaultInspectorConfigPlugin)
            .add_systems(EguiPrimaryContextPass, inspector_ui);
    }
}
