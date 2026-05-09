use bevy::prelude::*;

#[derive(Component, Debug, Default)]
pub struct Player;

#[derive(Component, Debug, Default)]
pub struct PrimarySceneCamera;

#[derive(Component, Debug, Default)]
pub struct AppSceneRoot;

#[derive(Component, Debug, Default)]
pub struct AppSceneEntity;

#[derive(Component, Debug, Default)]
pub struct ZooPet;

#[derive(Component, Debug, Default)]
pub struct ZooSceneRoot;

#[derive(Component, Debug, Default)]
pub struct ZooSceneEntity;

#[derive(Component, Debug, Default)]
pub struct ModelBrowserSceneRoot;

#[derive(Component, Debug, Default)]
pub struct ModelBrowserSceneEntity;

#[derive(Component, Debug, Default)]
pub struct ModelBrowserMetadataText;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ModelBrowserPageButtonAction {
    Back,
    Next,
}

#[derive(Component, Clone, Copy, Debug)]
pub struct ModelBrowserPageButton {
    pub action: ModelBrowserPageButtonAction,
}

impl ModelBrowserPageButton {
    pub const fn new(action: ModelBrowserPageButtonAction) -> Self {
        Self { action }
    }
}

#[derive(Component, Clone, Debug)]
pub struct BrowserAnimalModel {
    pub home_transform: Transform,
    pub showcase_y_offset: f32,
    pub filename: String,
}

impl BrowserAnimalModel {
    pub fn new(
        home_transform: Transform,
        showcase_y_offset: f32,
        filename: impl Into<String>,
    ) -> Self {
        Self {
            home_transform,
            showcase_y_offset,
            filename: filename.into(),
        }
    }
}

#[derive(Component, Clone, Copy, Debug)]
pub struct RotationComponent {
    pub radians_per_frame: Vec3,
}

impl RotationComponent {
    pub const fn new(radians_per_frame: Vec3) -> Self {
        Self { radians_per_frame }
    }
}

#[derive(Component, Debug)]
pub struct DebugHudText;

#[derive(Component, Debug)]
pub struct DebugHudFpsText;

#[derive(Component, Clone, Copy, Debug)]
pub struct DebugHudKeyText {
    pub key_code: KeyCode,
    pub is_toggle: bool,
}

impl DebugHudKeyText {
    pub const fn new(key_code: KeyCode, is_toggle: bool) -> Self {
        Self {
            key_code,
            is_toggle,
        }
    }
}

#[derive(Component, Debug)]
pub struct InspectorState {
    pub is_visible: bool,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Default for InspectorState {
    fn default() -> Self {
        Self {
            is_visible: false,
            x: 24.0,
            y: 156.0,
            width: 676.0,
            height: 620.0,
        }
    }
}
