use bevy::{
    asset::RenderAssetUsages,
    ecs::system::SystemParam,
    gltf::GltfAssetLabel,
    image::{CompressedImageFormats, ImageSampler, ImageType},
    light::CascadeShadowConfigBuilder,
    prelude::*,
    render::view::NoIndirectDrawing,
    text::{Underline, UnderlineColor},
    ui::UiScale,
    window::{
        Monitor, PrimaryWindow, WindowCloseRequested, WindowMoved, WindowResized, WindowResolution,
    },
};
use bevy_inspector_egui::{
    bevy_egui::{EguiContext, PrimaryEguiContext, egui},
    bevy_inspector,
    bevy_inspector::EntityFilter,
};
use bevy_persistent::prelude::Persistent;
use bevy_tween::prelude::{
    AnimationBuilderExt, Duration, EaseKind, IntoTarget, TransformTargetStateExt,
};
use bevy_zoo_game_shared::{
    GameTitle,
    window::{DEFAULT_WINDOW_HEIGHT, DEFAULT_WINDOW_WIDTH},
};

use crate::runtime::components::{
    AppSceneEntity, AppSceneRoot, BrowserAnimalModel, DebugHudFpsText, DebugHudKeyText,
    DebugHudText, InspectorState, ModelBrowserSceneEntity, ModelBrowserSceneRoot, Player,
    PrimarySceneCamera, RotationComponent, ZooPet, ZooSceneEntity, ZooSceneRoot,
};
use crate::runtime::resources::{
    ActiveScene, DebugHudInputStore, DebugHudState, GameTicks, MODEL_BROWSER_ANIMAL_PATHS,
    MODEL_BROWSER_CAMERA_Z, MODEL_BROWSER_GRID_COLUMNS, MODEL_BROWSER_GRID_ROWS,
    MODEL_BROWSER_GRID_SCALE, MODEL_BROWSER_GRID_SPACING, MODEL_BROWSER_MODEL_COUNT,
    MODEL_BROWSER_PICK_RADIUS, MODEL_BROWSER_SHOWCASE_SCALE, MODEL_BROWSER_SHOWCASE_Z,
    ModelBrowserSelection, PrimaryCameraDefaults, WindowPlacement, WindowPlacementState,
    WindowPlacementStore, ZooPetDefaults, ZooSceneDefaults, load_window_placement,
    valid_window_placement,
};

#[cfg(feature = "desktop-hot-reload")]
use crate::runtime::resources::desktop_hot_reload_patch_count;

mod glb_mesh;

const FPS_UPDATE_INTERVAL_SECONDS: f32 = 0.5;
const SCREEN_PADDING_TOP: f32 = 24.0;
const SCREEN_PADDING_LEFT: f32 = 24.0;
const TARGET_WIDTH: f32 = DEFAULT_WINDOW_WIDTH as f32;
const TARGET_HEIGHT: f32 = DEFAULT_WINDOW_HEIGHT as f32;
const DEBUG_HUD_FONT_SIZE: f32 = 22.0;
const DEBUG_WINDOW_FONT_SIZE: f32 = 14.0;
const ZOO_PLATFORM_GLB: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/Assets/Models/kenney_platformer-kit/Models/GLB format/block-grass-overhang-large.glb"
));
const ZOO_POLAR_GLB: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/Assets/Models/kenney_cube-pets_1.0/Models/GLB format/animal-polar.glb"
));
const ZOO_PET_COLORMAP_PNG: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/Assets/Models/kenney_cube-pets_1.0/Models/GLB format/Textures/colormap.png"
));
const ZOO_PLATFORMER_COLORMAP_PNG: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/Assets/Models/kenney_platformer-kit/Models/GLB format/Textures/colormap.png"
));
const ZOO_GRAVEYARD_COLORMAP_PNG: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/Assets/Models/kenney_graveyard-kit_5.0/Models/GLB format/Textures/colormap.png"
));
const ZOO_PINE_TREE_GLB: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/Assets/Models/kenney_graveyard-kit_5.0/Models/GLB format/pine-crooked.glb"
));
const ZOO_STAR_GLB: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/Assets/Models/kenney_platformer-kit/Models/GLB format/star.glb"
));
const STAR_ROTATION_PER_FRAME: Vec3 = Vec3::new(0.0, 0.01, 0.0);
const MODEL_BROWSER_SELECTED_ROTATION_PER_FRAME: Vec3 = Vec3::new(0.0, 0.01, 0.0);
const MODEL_BROWSER_ANIMAL_HALF_HEIGHTS: [f32; 24] = [
    0.810, 0.933, 1.064, 0.852, 0.933, 0.856, 0.866, 0.775, 1.056, 0.852, 0.862, 0.888, 0.905,
    0.916, 0.817, 0.791, 0.925, 0.856, 0.810, 0.775, 0.856, 0.852, 0.810, 0.831,
];

pub fn setup_game(mut commands: Commands) {
    commands.spawn((Player, Name::new(GameTitle::DISPLAY)));
}

pub fn setup_primary_camera(mut commands: Commands, camera_defaults: Res<PrimaryCameraDefaults>) {
    commands.spawn(primary_camera_bundle(&camera_defaults));
}

pub fn setup_scene_lighting(mut commands: Commands) {
    commands
        .spawn(three_point_lights_bundle())
        .with_children(spawn_three_point_light_children);
}

pub fn setup_app_scene(mut commands: Commands, camera_defaults: Res<PrimaryCameraDefaults>) {
    commands
        .spawn((
            Name::new("AppScene"),
            AppSceneRoot,
            AppSceneEntity,
            Transform::default(),
            GlobalTransform::default(),
            Visibility::Inherited,
        ))
        .with_children(|parent| {
            parent.spawn((primary_camera_bundle(&camera_defaults), AppSceneEntity));
            parent
                .spawn((three_point_lights_bundle(), AppSceneEntity))
                .with_children(|parent| {
                    parent.spawn((main_light_bundle(), AppSceneEntity));
                    parent.spawn((fill_light_bundle(), AppSceneEntity));
                    parent.spawn((back_light_bundle(), AppSceneEntity));
                });
            parent
                .spawn((debug_hud_bundle(), AppSceneEntity))
                .with_children(spawn_debug_hud_children);
        });
}

pub fn setup_zoo_scene(
    mut commands: Commands,
    pet_defaults: Res<ZooPetDefaults>,
    scene_defaults: Res<ZooSceneDefaults>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    spawn_zoo_scene(
        &mut commands,
        &pet_defaults,
        &scene_defaults,
        &mut meshes,
        &mut materials,
        &mut images,
    );
}

pub fn setup_model_browser_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_model_browser_scene(&mut commands, &asset_server);
}

pub fn toggle_scene_browser(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    asset_server: Res<AssetServer>,
    camera_defaults: Res<PrimaryCameraDefaults>,
    pet_defaults: Res<ZooPetDefaults>,
    scene_defaults: Res<ZooSceneDefaults>,
    mut active_scene: ResMut<ActiveScene>,
    mut selection: ResMut<ModelBrowserSelection>,
    mut camera_query: Query<(&mut Transform, &mut Projection), With<PrimarySceneCamera>>,
    zoo_scene_entities: Query<Entity, With<ZooSceneEntity>>,
    browser_scene_entities: Query<Entity, With<ModelBrowserSceneEntity>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    mut ticks: ResMut<GameTicks>,
) {
    if !keys.just_pressed(KeyCode::KeyB) {
        return;
    }

    selection.selected = None;
    ticks.0 = 0;

    match *active_scene {
        ActiveScene::GameScene => {
            despawn_scene_entities(&mut commands, &zoo_scene_entities);
            spawn_model_browser_scene(&mut commands, &asset_server);
            apply_browser_camera(&mut camera_query);
            *active_scene = ActiveScene::ModelBrowser;
        }
        ActiveScene::ModelBrowser => {
            despawn_scene_entities(&mut commands, &browser_scene_entities);
            apply_game_camera(&camera_defaults, &mut camera_query);
            spawn_zoo_scene(
                &mut commands,
                &pet_defaults,
                &scene_defaults,
                &mut meshes,
                &mut materials,
                &mut images,
            );
            *active_scene = ActiveScene::GameScene;
        }
    }
}

pub fn restart_zoo_scene(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    active_scene: Option<Res<ActiveScene>>,
    scene_entities: Query<Entity, With<ZooSceneEntity>>,
    pet_defaults: Res<ZooPetDefaults>,
    scene_defaults: Res<ZooSceneDefaults>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    mut ticks: ResMut<GameTicks>,
) {
    if !keys.just_pressed(KeyCode::KeyR) {
        return;
    }
    if active_scene.is_some_and(|scene| *scene == ActiveScene::ModelBrowser) {
        return;
    }

    reload_zoo_scene(
        &mut commands,
        &scene_entities,
        &pet_defaults,
        &scene_defaults,
        &mut meshes,
        &mut materials,
        &mut images,
        &mut ticks,
    );
}

pub fn model_browser_click_selection(
    mut commands: Commands,
    buttons: Res<ButtonInput<MouseButton>>,
    primary_window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<PrimarySceneCamera>>,
    mut selection: ResMut<ModelBrowserSelection>,
    model_query: Query<(Entity, &BrowserAnimalModel, &Transform)>,
) {
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }

    if let Some(selected) = selection.selected.take() {
        if let Ok((_, model, transform)) = model_query.get(selected) {
            tween_browser_model(
                &mut commands,
                selected,
                *transform,
                model.home_transform,
                EaseKind::CubicInOut,
                true,
            );
            commands.entity(selected).remove::<RotationComponent>();
        }
        return;
    }

    let Some(cursor_world) =
        cursor_world_position_on_browser_plane(&primary_window_query, &camera_query, 0.0)
    else {
        return;
    };

    let selected = model_query
        .iter()
        .min_by(|(_, _, left_transform), (_, _, right_transform)| {
            left_transform
                .translation
                .truncate()
                .distance_squared(cursor_world.truncate())
                .total_cmp(
                    &right_transform
                        .translation
                        .truncate()
                        .distance_squared(cursor_world.truncate()),
                )
        })
        .and_then(|(entity, model, transform)| {
            let distance = transform
                .translation
                .truncate()
                .distance(cursor_world.truncate());
            (distance <= MODEL_BROWSER_PICK_RADIUS).then_some((
                entity,
                *transform,
                model.showcase_y_offset,
            ))
        });

    let Some((entity, transform, showcase_y_offset)) = selected else {
        return;
    };

    tween_browser_model(
        &mut commands,
        entity,
        transform,
        browser_showcase_transform(showcase_y_offset),
        EaseKind::CubicOut,
        false,
    );
    commands.entity(entity).insert(RotationComponent::new(
        MODEL_BROWSER_SELECTED_ROTATION_PER_FRAME,
    ));
    selection.selected = Some(entity);
}

#[cfg(feature = "desktop-hot-reload")]
pub fn hot_reload_auto_restart_zoo_scene(
    mut last_seen_patch_count: Local<u64>,
    hud_state: Res<DebugHudState>,
    mut commands: Commands,
    scene_entities: Query<Entity, With<ZooSceneEntity>>,
    pet_defaults: Res<ZooPetDefaults>,
    scene_defaults: Res<ZooSceneDefaults>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    mut ticks: ResMut<GameTicks>,
) {
    let patch_count = desktop_hot_reload_patch_count();
    if patch_count == *last_seen_patch_count {
        return;
    }

    *last_seen_patch_count = patch_count;

    if !hud_state.is_hot_reload_autorestart_enabled {
        return;
    }

    reload_zoo_scene(
        &mut commands,
        &scene_entities,
        &pet_defaults,
        &scene_defaults,
        &mut meshes,
        &mut materials,
        &mut images,
        &mut ticks,
    );
}

#[cfg(not(feature = "desktop-hot-reload"))]
pub fn hot_reload_auto_restart_zoo_scene() {}

fn reload_zoo_scene(
    commands: &mut Commands,
    scene_entities: &Query<Entity, With<ZooSceneEntity>>,
    pet_defaults: &ZooPetDefaults,
    scene_defaults: &ZooSceneDefaults,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    images: &mut Assets<Image>,
    ticks: &mut GameTicks,
) {
    despawn_scene_entities(commands, scene_entities);

    ticks.0 = 0;
    spawn_zoo_scene(
        commands,
        pet_defaults,
        scene_defaults,
        meshes,
        materials,
        images,
    );
}

fn despawn_scene_entities<C: Component>(
    commands: &mut Commands,
    scene_entities: &Query<Entity, With<C>>,
) {
    for entity in scene_entities.iter() {
        commands.entity(entity).despawn();
    }
}

fn spawn_zoo_scene(
    commands: &mut Commands,
    pet_defaults: &ZooPetDefaults,
    scene_defaults: &ZooSceneDefaults,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    images: &mut Assets<Image>,
) {
    let pet_texture = texture_from_png(images, ZOO_PET_COLORMAP_PNG, "cube pets colormap");
    let platformer_texture =
        texture_from_png(images, ZOO_PLATFORMER_COLORMAP_PNG, "platformer colormap");
    let graveyard_texture =
        texture_from_png(images, ZOO_GRAVEYARD_COLORMAP_PNG, "graveyard colormap");

    commands
        .spawn((
            Name::new("GameScene"),
            ZooSceneRoot,
            ZooSceneEntity,
            Transform::default(),
            GlobalTransform::default(),
            Visibility::Inherited,
        ))
        .with_children(|parent| {
            spawn_glb_mesh(
                parent,
                meshes,
                materials,
                "Zoo Platform",
                ZOO_PLATFORM_GLB,
                scene_defaults.floor_model_path,
                scene_defaults.floor_transform,
                Color::WHITE,
                platformer_texture.clone(),
                None,
                None,
            );

            spawn_origin_cube(parent, meshes, materials);

            spawn_glb_mesh(
                parent,
                meshes,
                materials,
                "Zoo Pet Polar Bear",
                ZOO_POLAR_GLB,
                pet_defaults.model_path,
                pet_defaults.transform,
                Color::WHITE,
                pet_texture,
                Some(ZooPet),
                None,
            );

            spawn_glb_mesh(
                parent,
                meshes,
                materials,
                "Zoo Pine Tree",
                ZOO_PINE_TREE_GLB,
                scene_defaults.tree_model_path,
                scene_defaults.tree_transform,
                Color::WHITE,
                graveyard_texture,
                None,
                None,
            );

            spawn_glb_mesh(
                parent,
                meshes,
                materials,
                "Zoo Star",
                ZOO_STAR_GLB,
                "Models/kenney_platformer-kit/Models/GLB format/star.glb",
                Transform::from_translation(Vec3::new(0.0, 2.4, 1.6)).with_scale(Vec3::splat(0.28)),
                Color::WHITE,
                platformer_texture,
                None,
                Some(RotationComponent::new(STAR_ROTATION_PER_FRAME)),
            );
        });
}

fn spawn_model_browser_scene(commands: &mut Commands, asset_server: &AssetServer) {
    commands
        .spawn((
            Name::new("ModelBrowserScene"),
            ModelBrowserSceneRoot,
            ModelBrowserSceneEntity,
            Transform::default(),
            GlobalTransform::default(),
            Visibility::Inherited,
        ))
        .with_children(|parent| {
            for index in 0..MODEL_BROWSER_MODEL_COUNT {
                let transform = browser_grid_transform(index);
                let model_index = index % MODEL_BROWSER_ANIMAL_PATHS.len();
                let model_path = MODEL_BROWSER_ANIMAL_PATHS[model_index];
                let scene_handle =
                    asset_server.load(GltfAssetLabel::Scene(0).from_asset(model_path));
                parent.spawn((
                    Name::new(format!("Browser Animal {:02}", index + 1)),
                    ModelBrowserSceneEntity,
                    BrowserAnimalModel::new(transform, browser_showcase_y_offset(model_index)),
                    SceneRoot(scene_handle),
                    transform,
                ));
            }
        });
}

fn browser_grid_transform(index: usize) -> Transform {
    let column = index % MODEL_BROWSER_GRID_COLUMNS;
    let row = index / MODEL_BROWSER_GRID_COLUMNS;
    let x = (column as f32 - (MODEL_BROWSER_GRID_COLUMNS as f32 - 1.0) * 0.5)
        * MODEL_BROWSER_GRID_SPACING;
    let y =
        ((MODEL_BROWSER_GRID_ROWS as f32 - 1.0) * 0.5 - row as f32) * MODEL_BROWSER_GRID_SPACING;

    Transform::from_xyz(x, y, 0.0).with_scale(Vec3::splat(MODEL_BROWSER_GRID_SCALE))
}

fn browser_showcase_y_offset(model_index: usize) -> f32 {
    -MODEL_BROWSER_ANIMAL_HALF_HEIGHTS[model_index] * MODEL_BROWSER_SHOWCASE_SCALE
}

fn browser_showcase_transform(y_offset: f32) -> Transform {
    Transform::from_xyz(0.0, y_offset, MODEL_BROWSER_SHOWCASE_Z)
        .with_scale(Vec3::splat(MODEL_BROWSER_SHOWCASE_SCALE))
}

fn tween_browser_model(
    commands: &mut Commands,
    entity: Entity,
    current: Transform,
    target: Transform,
    ease: EaseKind,
    animate_rotation: bool,
) {
    let target_component = entity.into_target();
    let mut transform_state = target_component.transform_state(current);
    if animate_rotation {
        commands.entity(entity).animation().insert_tween_here(
            Duration::from_millis(360),
            ease,
            (
                transform_state.translation_to(target.translation),
                transform_state.scale_to(target.scale),
                transform_state.rotation_to(target.rotation),
            ),
        );
    } else {
        commands.entity(entity).animation().insert_tween_here(
            Duration::from_millis(360),
            ease,
            (
                transform_state.translation_to(target.translation),
                transform_state.scale_to(target.scale),
            ),
        );
    }
}

fn cursor_world_position_on_browser_plane(
    primary_window_query: &Query<&Window, With<PrimaryWindow>>,
    camera_query: &Query<(&Camera, &GlobalTransform), With<PrimarySceneCamera>>,
    plane_z: f32,
) -> Option<Vec3> {
    let window = primary_window_query.single().ok()?;
    let cursor_position = window.cursor_position()?;
    let (camera, camera_transform) = camera_query.single().ok()?;
    let ray = camera
        .viewport_to_world(camera_transform, cursor_position)
        .ok()?;
    let denominator = ray.direction.z;

    if denominator.abs() < f32::EPSILON {
        return None;
    }

    let distance = (plane_z - ray.origin.z) / denominator;
    (distance >= 0.0).then_some(ray.origin + ray.direction * distance)
}

fn spawn_origin_cube(
    parent: &mut ChildSpawnerCommands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    let mesh = meshes.add(Cuboid::from_size(Vec3::ONE));
    let material = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.0, 0.0),
        unlit: false,
        perceptual_roughness: 0.82,
        ..Default::default()
    });

    parent.spawn((
        Name::new("oriigin"),
        ZooSceneEntity,
        Mesh3d(mesh),
        MeshMaterial3d(material),
        Transform::from_translation(Vec3::ZERO).with_scale(Vec3::ONE),
    ));
}

fn primary_camera_bundle(camera_defaults: &PrimaryCameraDefaults) -> impl Bundle {
    (
        Name::new("Primary 3D Camera"),
        PrimarySceneCamera,
        Camera3d::default(),
        NoIndirectDrawing,
        Projection::Perspective(PerspectiveProjection {
            fov: camera_defaults.fov_radians,
            near: camera_defaults.near,
            far: camera_defaults.far,
            ..Default::default()
        }),
        camera_defaults.transform(),
    )
}

fn apply_game_camera(
    camera_defaults: &PrimaryCameraDefaults,
    camera_query: &mut Query<(&mut Transform, &mut Projection), With<PrimarySceneCamera>>,
) {
    set_primary_camera_view(
        camera_query,
        camera_defaults.transform(),
        camera_defaults.fov_radians,
        camera_defaults.near,
        camera_defaults.far,
    );
}

fn apply_browser_camera(
    camera_query: &mut Query<(&mut Transform, &mut Projection), With<PrimarySceneCamera>>,
) {
    set_primary_camera_view(
        camera_query,
        Transform::from_xyz(0.0, 0.0, MODEL_BROWSER_CAMERA_Z)
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        std::f32::consts::FRAC_PI_4,
        0.1,
        1000.0,
    );
}

fn set_primary_camera_view(
    camera_query: &mut Query<(&mut Transform, &mut Projection), With<PrimarySceneCamera>>,
    transform: Transform,
    fov_radians: f32,
    near: f32,
    far: f32,
) {
    let Ok((mut camera_transform, mut projection)) = camera_query.single_mut() else {
        return;
    };

    *camera_transform = transform;
    *projection = Projection::Perspective(PerspectiveProjection {
        fov: fov_radians,
        near,
        far,
        ..Default::default()
    });
}

fn three_point_lights_bundle() -> impl Bundle {
    (
        Name::new("ThreePointLights"),
        Transform::default(),
        GlobalTransform::default(),
        Visibility::Inherited,
    )
}

fn spawn_three_point_light_children(parent: &mut ChildSpawnerCommands) {
    parent.spawn(main_light_bundle());
    parent.spawn(fill_light_bundle());
    parent.spawn(back_light_bundle());
}

fn main_light_bundle() -> impl Bundle {
    (
        Name::new("Main Light"),
        DirectionalLight {
            illuminance: 8_820.0,
            shadows_enabled: true,
            ..Default::default()
        },
        CascadeShadowConfigBuilder {
            num_cascades: 1,
            maximum_distance: 18.0,
            ..Default::default()
        }
        .build(),
        Transform::from_xyz(-3.5, 5.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
    )
}

fn fill_light_bundle() -> impl Bundle {
    (
        Name::new("Fill Light"),
        DirectionalLight {
            illuminance: 6_200.0,
            shadows_enabled: false,
            ..Default::default()
        },
        Transform::from_xyz(4.0, 4.2, 3.5).looking_at(Vec3::ZERO, Vec3::Y),
    )
}

fn back_light_bundle() -> impl Bundle {
    (
        Name::new("Back Light"),
        DirectionalLight {
            illuminance: 7_000.0,
            shadows_enabled: true,
            ..Default::default()
        },
        Transform::from_xyz(3.0, 4.8, -4.0).looking_at(Vec3::ZERO, Vec3::Y),
    )
}

fn spawn_glb_mesh(
    parent: &mut ChildSpawnerCommands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    name: &'static str,
    glb: &[u8],
    model_path: &str,
    transform: Transform,
    base_color: Color,
    base_color_texture: Option<Handle<Image>>,
    zoo_pet: Option<ZooPet>,
    rotation: Option<RotationComponent>,
) {
    let mesh = match glb_mesh::mesh_from_glb(glb) {
        Ok(mesh) => meshes.add(mesh),
        Err(error) => {
            warn!("Failed to load zoo scene model {model_path}: {error}");
            return;
        }
    };
    let material = materials.add(StandardMaterial {
        base_color,
        base_color_texture,
        unlit: false,
        perceptual_roughness: 0.82,
        ..Default::default()
    });

    let mut entity = parent.spawn((
        Name::new(name),
        ZooSceneEntity,
        Mesh3d(mesh),
        MeshMaterial3d(material),
        transform,
    ));

    if let Some(zoo_pet) = zoo_pet {
        entity.insert(zoo_pet);
    }

    if let Some(rotation) = rotation {
        entity.insert(rotation);
    }
}

fn texture_from_png(
    images: &mut Assets<Image>,
    source: &[u8],
    texture_name: &'static str,
) -> Option<Handle<Image>> {
    match Image::from_buffer(
        source,
        ImageType::Extension("png"),
        CompressedImageFormats::empty(),
        true,
        ImageSampler::nearest(),
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    ) {
        Ok(image) => Some(images.add(image)),
        Err(error) => {
            warn!("Failed to load texture {texture_name}: {error}");
            None
        }
    }
}

pub fn load_saved_window_placement(
    mut placement_state: ResMut<WindowPlacementState>,
    persistent_placement: Option<Res<Persistent<WindowPlacementStore>>>,
) {
    placement_state.current = persistent_placement
        .and_then(|persistent_placement| {
            valid_window_placement(persistent_placement.current.clone())
        })
        .or_else(load_window_placement);
}

pub fn load_saved_debug_hud_input(
    mut hud_state: ResMut<DebugHudState>,
    persistent_input: Option<Res<Persistent<DebugHudInputStore>>>,
) {
    if let Some(persistent_input) = persistent_input {
        persistent_input.apply_to_state(&mut hud_state);
    }
}

pub fn advance_ticks(mut ticks: ResMut<GameTicks>) {
    ticks.0 += 1;
}

pub fn rotation_system(mut query: Query<(&RotationComponent, &mut Transform)>) {
    for (rotation, mut transform) in &mut query {
        transform.rotate(Quat::from_euler(
            EulerRot::XYZ,
            rotation.radians_per_frame.x,
            rotation.radians_per_frame.y,
            rotation.radians_per_frame.z,
        ));
    }
}

pub fn setup_inspector(mut commands: Commands, hud_state: Res<DebugHudState>) {
    let mut inspector = InspectorState::default();
    inspector.is_visible = hud_state.is_inspector_visible;

    commands.spawn((Name::new("Bevy Inspector"), inspector));
}

pub fn setup_debug_hud(mut commands: Commands) {
    commands
        .spawn(debug_hud_bundle())
        .with_children(spawn_debug_hud_children);
}

fn debug_hud_bundle() -> impl Bundle {
    (
        Text::new("Bevy Zoo Game\nFrame: 0\nKEYS: "),
        TextFont {
            font_size: DEBUG_HUD_FONT_SIZE,
            ..Default::default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(SCREEN_PADDING_LEFT),
            top: Val::Px(SCREEN_PADDING_TOP),
            width: Val::Px(260.0),
            align_items: AlignItems::Center,
            padding: UiRect {
                left: Val::Px(40.0),
                right: Val::Px(12.0),
                top: Val::Px(8.0),
                bottom: Val::Px(8.0),
            },
            border_radius: BorderRadius::all(Val::Px(8.0)),
            ..Default::default()
        },
        BackgroundColor(Color::srgba(0.02, 0.02, 0.02, 0.72)),
        Visibility::Visible,
        DebugHudText,
    )
}

fn spawn_debug_hud_children(parent: &mut ChildSpawnerCommands) {
    spawn_key_span(parent, "W", KeyCode::KeyW, false);
    spawn_key_span(parent, "A", KeyCode::KeyA, false);
    spawn_key_span(parent, "S", KeyCode::KeyS, false);
    spawn_key_span(parent, "D", KeyCode::KeyD, false);
    parent.spawn((TextSpan::new(", "), debug_hud_text_font()));
    spawn_key_span(parent, "R", KeyCode::KeyR, false);
    parent.spawn((TextSpan::new("\nKEYS: "), debug_hud_text_font()));
    spawn_key_span(parent, "F", KeyCode::KeyF, true);
    parent.spawn((TextSpan::new(", "), debug_hud_text_font()));
    spawn_key_span(parent, "I", KeyCode::KeyI, true);
    parent.spawn((TextSpan::new(", "), debug_hud_text_font()));
    spawn_key_span(parent, "H", KeyCode::KeyH, true);
    parent.spawn((TextSpan::new(", "), debug_hud_text_font()));
    spawn_key_span(parent, "B", KeyCode::KeyB, true);
    parent.spawn((TextSpan::new(""), debug_hud_text_font(), DebugHudFpsText));
}

#[derive(SystemParam)]
pub struct DebugHudUpdateParams<'w, 's> {
    keys: Res<'w, ButtonInput<KeyCode>>,
    time: Res<'w, Time>,
    ticks: Res<'w, GameTicks>,
    active_scene: Option<Res<'w, ActiveScene>>,
    hud_state: ResMut<'w, DebugHudState>,
    text_query: Query<'w, 's, &'static mut Text, With<DebugHudText>>,
    fps_text_query: Query<'w, 's, &'static mut TextSpan, With<DebugHudFpsText>>,
    key_text_query: Query<'w, 's, (&'static DebugHudKeyText, &'static mut UnderlineColor)>,
}

pub fn update_debug_hud(mut params: DebugHudUpdateParams) {
    params.hud_state.fps_accumulated_seconds += params.time.delta_secs();
    params.hud_state.fps_accumulated_frames += 1;

    if params.hud_state.fps_accumulated_seconds >= FPS_UPDATE_INTERVAL_SECONDS {
        params.hud_state.fps_display_value = if params.hud_state.fps_accumulated_seconds > 0.0 {
            params.hud_state.fps_accumulated_frames as f32
                / params.hud_state.fps_accumulated_seconds
        } else {
            0.0
        };

        params.hud_state.fps_accumulated_seconds = 0.0;
        params.hud_state.fps_accumulated_frames = 0;
    }

    let fps_on = params.hud_state.is_fps_visible;
    let inspector_on = params.hud_state.is_inspector_visible;
    let hot_reload_autorestart_on = params.hud_state.is_hot_reload_autorestart_enabled;
    let model_browser_on = params
        .active_scene
        .as_deref()
        .is_some_and(|scene| *scene == ActiveScene::ModelBrowser);

    for (key_text, mut underline_color) in &mut params.key_text_query {
        let is_active = if key_text.is_toggle {
            match key_text.key_code {
                KeyCode::KeyF => fps_on,
                KeyCode::KeyI => inspector_on,
                KeyCode::KeyH => hot_reload_autorestart_on,
                KeyCode::KeyB => model_browser_on,
                _ => false,
            }
        } else {
            params.keys.pressed(key_text.key_code)
        };

        underline_color.0 = if is_active {
            Color::WHITE
        } else {
            Color::srgba(1.0, 1.0, 1.0, 0.0)
        };
    }

    let full_text = format!("Bevy Zoo Game\nFrame: {}\nKEYS: ", params.ticks.0);
    for mut text in &mut params.text_query {
        *text = Text::new(full_text.clone());
    }

    let fps_line = if params.hud_state.is_fps_visible {
        format!("\nFPS: {:.1}", params.hud_state.fps_display_value)
    } else {
        String::new()
    };

    for mut fps_text in &mut params.fps_text_query {
        *fps_text = TextSpan::new(fps_line.clone());
    }
}

pub fn toggle_debug_hud_inputs(
    keys: Res<ButtonInput<KeyCode>>,
    mut hud_state: ResMut<DebugHudState>,
    mut inspector_query: Query<&mut InspectorState>,
    mut persistent_input: Option<ResMut<Persistent<DebugHudInputStore>>>,
) {
    let mut changed = false;

    if keys.just_pressed(KeyCode::KeyF) {
        hud_state.is_fps_visible = !hud_state.is_fps_visible;
        changed = true;
    }

    if keys.just_pressed(KeyCode::KeyH) {
        hud_state.is_hot_reload_autorestart_enabled = !hud_state.is_hot_reload_autorestart_enabled;
        changed = true;
    }

    if keys.just_pressed(KeyCode::KeyI) {
        hud_state.is_inspector_visible = !hud_state.is_inspector_visible;
        if let Ok(mut inspector) = inspector_query.single_mut() {
            inspector.is_visible = hud_state.is_inspector_visible;
        }
        changed = true;
    }

    if !changed {
        return;
    }

    let Some(ref mut persistent_input) = persistent_input else {
        warn!("Failed to save DebugHUD input: persistent store unavailable");
        return;
    };

    if let Err(error) = persistent_input.set(DebugHudInputStore::from_state(&hud_state)) {
        warn!("Failed to save DebugHUD input: {error}");
    }
}

pub fn scale_debug_hud(
    mut window_resized_events: Option<MessageReader<WindowResized>>,
    primary_window_query: Query<(Entity, &Window), With<PrimaryWindow>>,
    mut ui_scale: Option<ResMut<UiScale>>,
) {
    let Some(ref mut window_resized_events) = window_resized_events else {
        return;
    };
    let Some(ref mut ui_scale) = ui_scale else {
        return;
    };
    let Ok((primary_window_entity, primary_window)) = primary_window_query.single() else {
        return;
    };

    let mut primary_window_resized = false;
    for resized_event in window_resized_events.read() {
        if resized_event.window == primary_window_entity {
            primary_window_resized = true;
        }
    }

    if !primary_window_resized {
        return;
    }

    let width_scale = primary_window.resolution.width() / TARGET_WIDTH;
    let height_scale = primary_window.resolution.height() / TARGET_HEIGHT;
    ui_scale.0 = width_scale.min(height_scale).max(0.1);
}

pub fn restore_window_placement_to_current_monitors(
    mut placement_state: ResMut<WindowPlacementState>,
    mut primary_window_query: Query<&mut Window, With<PrimaryWindow>>,
    monitor_query: Query<&Monitor>,
) {
    if placement_state.restored {
        return;
    }
    if monitor_query.iter().next().is_none() {
        return;
    }

    let Some(saved_placement) = placement_state.current.clone() else {
        placement_state.restored = true;
        return;
    };

    let Ok(mut window) = primary_window_query.single_mut() else {
        return;
    };

    if let Some(restored_position) = restored_position(&monitor_query, &saved_placement) {
        window.resolution =
            restored_window_resolution(&window.resolution, saved_placement.window_size);
        window.position = WindowPosition::At(restored_position);
    } else {
        apply_primary_centered_fallback(&mut window);
    }

    placement_state.restored = true;
}

pub fn track_window_placement(
    mut window_moved_events: Option<MessageReader<WindowMoved>>,
    primary_window_query: Query<(Entity, &Window), With<PrimaryWindow>>,
    monitor_query: Query<&Monitor>,
    mut placement_state: ResMut<WindowPlacementState>,
) {
    let Some(ref mut window_moved_events) = window_moved_events else {
        return;
    };
    let Ok((primary_window_entity, primary_window)) = primary_window_query.single() else {
        return;
    };

    for moved_event in window_moved_events.read() {
        if moved_event.window != primary_window_entity {
            continue;
        }

        placement_state.current = placement_for_window(
            moved_event.position,
            logical_window_size(primary_window),
            primary_window.resolution.physical_size(),
            &monitor_query,
        );
    }
}

pub fn track_window_size(
    mut window_resized_events: Option<MessageReader<WindowResized>>,
    primary_window_query: Query<(Entity, &Window), With<PrimaryWindow>>,
    monitor_query: Query<&Monitor>,
    mut placement_state: ResMut<WindowPlacementState>,
) {
    let Some(ref mut window_resized_events) = window_resized_events else {
        return;
    };
    let Ok((primary_window_entity, primary_window)) = primary_window_query.single() else {
        return;
    };

    for resized_event in window_resized_events.read() {
        if resized_event.window != primary_window_entity {
            continue;
        }

        let window_position = placement_state
            .current
            .as_ref()
            .map(|placement| placement.window_position)
            .or_else(|| match primary_window.position {
                WindowPosition::At(position) => Some(position),
                WindowPosition::Automatic | WindowPosition::Centered(_) => None,
            });

        let Some(window_position) = window_position else {
            continue;
        };

        placement_state.current = placement_for_window(
            window_position,
            logical_size_from_resize(resized_event),
            primary_window.resolution.physical_size(),
            &monitor_query,
        );
    }
}

pub fn save_window_placement_on_close(
    mut close_requested_events: Option<MessageReader<WindowCloseRequested>>,
    primary_window_query: Query<(Entity, &Window), With<PrimaryWindow>>,
    monitor_query: Query<&Monitor>,
    placement_state: Res<WindowPlacementState>,
    mut persistent_placement: Option<ResMut<Persistent<WindowPlacementStore>>>,
) {
    let Some(ref mut close_requested_events) = close_requested_events else {
        return;
    };
    let Ok((primary_window_entity, window)) = primary_window_query.single() else {
        return;
    };

    let should_save = close_requested_events
        .read()
        .any(|event| event.window == primary_window_entity);

    if !should_save {
        return;
    }

    let current_window_placement = match window.position {
        WindowPosition::At(position) => placement_for_window(
            position,
            logical_window_size(window),
            window.resolution.physical_size(),
            &monitor_query,
        ),
        WindowPosition::Automatic | WindowPosition::Centered(_) => None,
    };

    let placement_with_current_size = placement_state.current.as_ref().map(|placement| {
        placement_with_current_window_size(
            placement,
            logical_window_size(window),
            window.resolution.physical_size(),
            &monitor_query,
        )
    });
    let placement = current_window_placement
        .or(placement_with_current_size)
        .or_else(|| placement_state.current.clone());

    let Some(placement) = placement else {
        return;
    };

    let Some(ref mut persistent_placement) = persistent_placement else {
        warn!("Failed to save window placement: persistent store unavailable");
        return;
    };

    if let Err(error) = persistent_placement.set(WindowPlacementStore {
        current: Some(placement),
    }) {
        warn!("Failed to save window placement: {error}");
    }
}

pub fn inspector_ui(world: &mut World) {
    let Some((is_visible, x, y, width, height)) = inspector_window_settings(world) else {
        return;
    };

    if !is_visible {
        return;
    }

    let Ok(mut egui_context) = world
        .query_filtered::<&mut EguiContext, With<PrimaryEguiContext>>()
        .single(world)
        .cloned()
    else {
        return;
    };

    let egui_context = egui_context.get_mut();
    use_matching_debug_window_text_style(egui_context);

    egui::Window::new("Bevy Inspector")
        .default_pos(egui::pos2(x, y))
        .default_size(egui::vec2(width, height))
        .show(egui_context, |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                ui.heading("Bevy Zoo Game");
                bevy_inspector::ui_for_entities_filtered(world, ui, true, &InspectorEntityFilter);
                ui.allocate_space(ui.available_size());
            });
        });
}

fn use_matching_debug_window_text_style(context: &egui::Context) {
    let mut style = (*context.style()).clone();
    let font_id = egui::FontId::proportional(DEBUG_WINDOW_FONT_SIZE);

    for text_style in style.text_styles.values_mut() {
        *text_style = font_id.clone();
    }

    context.set_style(style);
}

fn spawn_key_span(
    parent: &mut ChildSpawnerCommands,
    text: &'static str,
    key_code: KeyCode,
    is_toggle: bool,
) {
    parent.spawn((
        TextSpan::new(text),
        debug_hud_text_font(),
        Underline,
        UnderlineColor(Color::srgba(1.0, 1.0, 1.0, 0.0)),
        DebugHudKeyText::new(key_code, is_toggle),
    ));
}

fn debug_hud_text_font() -> TextFont {
    TextFont {
        font_size: DEBUG_HUD_FONT_SIZE,
        ..Default::default()
    }
}

struct InspectorEntityFilter;

impl EntityFilter for InspectorEntityFilter {
    type StaticFilter = ();

    fn filter_entity(&self, world: &mut World, entity: Entity) -> bool {
        world.get::<Name>(entity).is_some()
    }
}

fn inspector_window_settings(world: &mut World) -> Option<(bool, f32, f32, f32, f32)> {
    let mut query = world.query::<&InspectorState>();
    let inspector = query.iter(world).next()?;
    Some((
        inspector.is_visible,
        inspector.x,
        inspector.y,
        inspector.width,
        inspector.height,
    ))
}

fn placement_for_window(
    window_position: IVec2,
    logical_window_size: UVec2,
    physical_window_size: UVec2,
    monitor_query: &Query<&Monitor>,
) -> Option<WindowPlacement> {
    let monitor = monitor_query
        .iter()
        .max_by_key(|monitor| {
            window_monitor_overlap_area(monitor, window_position, physical_window_size)
        })
        .or_else(|| monitor_query.iter().next())?;

    Some(WindowPlacement {
        window_position,
        window_size: logical_window_size,
        monitor_name: monitor.name.clone(),
        monitor_position: monitor.physical_position,
        monitor_size: monitor.physical_size(),
        relative_position: window_position - monitor.physical_position,
    })
}

fn placement_with_current_window_size(
    saved_placement: &WindowPlacement,
    current_logical_window_size: UVec2,
    current_physical_window_size: UVec2,
    monitor_query: &Query<&Monitor>,
) -> WindowPlacement {
    placement_for_window(
        saved_placement.window_position,
        current_logical_window_size,
        current_physical_window_size,
        monitor_query,
    )
    .unwrap_or_else(|| {
        saved_placement_with_current_window_size(saved_placement, current_logical_window_size)
    })
}

fn saved_placement_with_current_window_size(
    saved_placement: &WindowPlacement,
    current_logical_window_size: UVec2,
) -> WindowPlacement {
    let mut placement = saved_placement.clone();
    placement.window_size = current_logical_window_size;
    placement
}

fn window_monitor_overlap_area(
    monitor: &Monitor,
    window_position: IVec2,
    physical_window_size: UVec2,
) -> i64 {
    let monitor_min = monitor.physical_position;
    let monitor_max = monitor_min + monitor.physical_size().as_ivec2();
    let window_max = window_position + physical_window_size.as_ivec2();

    let overlap_width =
        (window_max.x.min(monitor_max.x) - window_position.x.max(monitor_min.x)).max(0);
    let overlap_height =
        (window_max.y.min(monitor_max.y) - window_position.y.max(monitor_min.y)).max(0);

    i64::from(overlap_width) * i64::from(overlap_height)
}

fn monitor_overlaps_window(monitor: &Monitor, window_position: IVec2, window_size: UVec2) -> bool {
    window_monitor_overlap_area(monitor, window_position, window_size) > 0
}

fn logical_window_size(window: &Window) -> UVec2 {
    UVec2::new(
        window.resolution.width().round().max(1.0) as u32,
        window.resolution.height().round().max(1.0) as u32,
    )
}

fn logical_size_from_resize(resized_event: &WindowResized) -> UVec2 {
    UVec2::new(
        resized_event.width.round().max(1.0) as u32,
        resized_event.height.round().max(1.0) as u32,
    )
}

fn restored_window_resolution(
    current_resolution: &WindowResolution,
    saved_logical_size: UVec2,
) -> WindowResolution {
    let mut resolution = current_resolution.clone();
    resolution.set(saved_logical_size.x as f32, saved_logical_size.y as f32);
    resolution
}

fn restored_position(
    monitor_query: &Query<&Monitor>,
    saved_placement: &WindowPlacement,
) -> Option<IVec2> {
    if monitor_query.iter().any(|monitor| {
        monitor_overlaps_window(
            monitor,
            saved_placement.window_position,
            estimated_physical_window_size(saved_placement, monitor),
        )
    }) {
        return Some(saved_placement.window_position);
    }

    let monitor = find_matching_monitor(monitor_query, saved_placement)?;
    let remapped_position = monitor.physical_position + saved_placement.relative_position;

    if monitor_overlaps_window(
        monitor,
        remapped_position,
        estimated_physical_window_size(saved_placement, monitor),
    ) {
        Some(remapped_position)
    } else {
        None
    }
}

fn estimated_physical_window_size(placement: &WindowPlacement, monitor: &Monitor) -> UVec2 {
    let scale_factor = monitor.scale_factor.max(1.0) as f32;
    UVec2::new(
        (placement.window_size.x as f32 * scale_factor)
            .round()
            .max(1.0) as u32,
        (placement.window_size.y as f32 * scale_factor)
            .round()
            .max(1.0) as u32,
    )
}

fn apply_primary_centered_fallback(window: &mut Window) {
    window.resolution = WindowResolution::new(DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT);
    window.position = WindowPosition::Centered(MonitorSelection::Primary);
}

fn find_matching_monitor<'a>(
    monitor_query: &'a Query<&Monitor>,
    saved_placement: &WindowPlacement,
) -> Option<&'a Monitor> {
    monitor_query
        .iter()
        .find(|monitor| {
            monitor.name == saved_placement.monitor_name
                && monitor.physical_size() == saved_placement.monitor_size
        })
        .or_else(|| {
            monitor_query
                .iter()
                .find(|monitor| monitor.name == saved_placement.monitor_name)
        })
        .or_else(|| {
            monitor_query
                .iter()
                .find(|monitor| monitor.physical_position == saved_placement.monitor_position)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug_window_text_styles_use_matching_font_face_and_size() {
        let context = egui::Context::default();

        use_matching_debug_window_text_style(&context);

        let style = context.style();
        let expected_font_id = egui::FontId::proportional(DEBUG_WINDOW_FONT_SIZE);

        assert!(
            style
                .text_styles
                .values()
                .all(|font_id| font_id.family == expected_font_id.family
                    && font_id.size == expected_font_id.size)
        );
    }

    #[test]
    fn debug_hud_text_spans_use_matching_font_size() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_systems(Startup, setup_debug_hud);

        app.update();

        let mut hud_query = app
            .world_mut()
            .query_filtered::<(Entity, &TextFont), With<DebugHudText>>();
        let (hud_entity, hud_font) = hud_query.single(app.world()).unwrap();
        assert_eq!(hud_font.font_size, DEBUG_HUD_FONT_SIZE);

        let children = app.world().get::<Children>(hud_entity).unwrap();
        assert!(!children.is_empty());

        for child in children.iter() {
            let child_font = app.world().get::<TextFont>(child).unwrap();
            assert_eq!(child_font.font_size, DEBUG_HUD_FONT_SIZE);
        }
    }

    #[test]
    fn debug_hud_toggle_keys_are_on_toggle_line() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_systems(Startup, setup_debug_hud);

        app.update();

        let mut hud_query = app
            .world_mut()
            .query_filtered::<(Entity, &Text), With<DebugHudText>>();
        let (hud_entity, hud_text) = hud_query.single(app.world()).unwrap();
        let children = app.world().get::<Children>(hud_entity).unwrap();
        let mut rendered_text = hud_text.to_string();

        for child in children.iter() {
            if let Some(span) = app.world().get::<TextSpan>(child) {
                rendered_text.push_str(span.as_str());
            }
        }

        assert_eq!(
            rendered_text,
            "Bevy Zoo Game\nFrame: 0\nKEYS: WASD, R\nKEYS: F, I, H, B"
        );
    }

    #[test]
    fn restored_resolution_applies_saved_size_as_logical_units() {
        let mut current_resolution = WindowResolution::new(1024, 768);
        current_resolution.set_scale_factor(1.5);

        let restored = restored_window_resolution(&current_resolution, UVec2::new(512, 384));

        assert_eq!(restored.width(), 512.0);
        assert_eq!(restored.height(), 384.0);
        assert_eq!(restored.physical_width(), 768);
        assert_eq!(restored.physical_height(), 576);
    }

    #[test]
    fn scene_lighting_uses_three_point_rig() {
        let mut app = App::new();
        app.add_systems(Startup, setup_scene_lighting);

        app.update();

        let light_children = {
            let mut light_root_query = app.world_mut().query::<(&Name, &Children)>();
            light_root_query
                .iter(app.world())
                .find(|(name, _)| name.as_str() == "ThreePointLights")
                .map(|(_, children)| children.iter().collect::<Vec<_>>())
                .unwrap()
        };
        let light_child_names = light_children
            .iter()
            .map(|child| {
                app.world()
                    .get::<Name>(*child)
                    .unwrap()
                    .as_str()
                    .to_string()
            })
            .collect::<Vec<_>>();
        assert_eq!(
            light_child_names,
            vec!["Main Light", "Fill Light", "Back Light"]
        );

        let mut directional_lights = app
            .world_mut()
            .query::<(&Name, &DirectionalLight, &Transform)>();
        let directional_lights = directional_lights
            .iter(app.world())
            .map(|(name, light, transform)| {
                (
                    name.as_str(),
                    light.illuminance,
                    light.shadows_enabled,
                    transform.translation,
                )
            })
            .collect::<Vec<_>>();
        assert!(directional_lights.contains(&(
            "Main Light",
            8_820.0,
            true,
            Vec3::new(-3.5, 5.0, 4.0)
        )));
        assert!(directional_lights.contains(&(
            "Fill Light",
            6_200.0,
            false,
            Vec3::new(4.0, 4.2, 3.5)
        )));
        assert!(directional_lights.contains(&(
            "Back Light",
            7_000.0,
            true,
            Vec3::new(3.0, 4.8, -4.0)
        )));
        assert_eq!(directional_lights.len(), 3);

        let mut point_lights = app.world_mut().query::<&PointLight>();
        assert_eq!(point_lights.iter(app.world()).count(), 0);

        let mut shadow_configs = app.world_mut().query::<&bevy::light::CascadeShadowConfig>();
        assert_eq!(shadow_configs.iter(app.world()).count(), 3);
    }

    #[test]
    fn zoo_scene_materials_are_lit() {
        let mut app = App::new();
        app.init_resource::<Assets<Mesh>>()
            .init_resource::<Assets<StandardMaterial>>()
            .init_resource::<Assets<Image>>()
            .init_resource::<PrimaryCameraDefaults>()
            .init_resource::<ZooPetDefaults>()
            .init_resource::<ZooSceneDefaults>()
            .add_systems(Startup, setup_zoo_scene);

        app.update();

        let materials = app.world().resource::<Assets<StandardMaterial>>();
        assert!(!materials.is_empty());
        assert!(materials.iter().all(|(_, material)| !material.unlit));
    }

    #[test]
    fn app_scene_groups_shared_camera_lights_and_hud_under_scene_root() {
        let mut app = App::new();
        app.init_resource::<PrimaryCameraDefaults>()
            .add_systems(Startup, setup_app_scene);

        app.update();

        let mut root_query = app
            .world_mut()
            .query_filtered::<Entity, With<AppSceneRoot>>();
        let root = root_query.single(app.world()).unwrap();
        assert_eq!(
            app.world().get::<Visibility>(root),
            Some(&Visibility::Inherited)
        );
        assert!(app.world().get::<InheritedVisibility>(root).is_some());
        let children = app.world().get::<Children>(root).unwrap();
        assert_eq!(children.len(), 3);

        let light_root = children
            .iter()
            .find(|child| {
                app.world()
                    .get::<Name>(*child)
                    .is_some_and(|name| name.as_str() == "ThreePointLights")
            })
            .unwrap();
        let light_children = app.world().get::<Children>(light_root).unwrap();
        let light_child_names = light_children
            .iter()
            .map(|child| app.world().get::<Name>(child).unwrap().as_str().to_string())
            .collect::<Vec<_>>();
        assert_eq!(
            light_child_names,
            vec!["Main Light", "Fill Light", "Back Light"]
        );

        let mut camera_query = app
            .world_mut()
            .query_filtered::<Entity, With<PrimarySceneCamera>>();
        assert_eq!(camera_query.iter(app.world()).count(), 1);
        let mut no_indirect_camera_query = app
            .world_mut()
            .query_filtered::<Entity, (With<PrimarySceneCamera>, With<NoIndirectDrawing>)>();
        assert_eq!(no_indirect_camera_query.iter(app.world()).count(), 1);

        let mut hud_query = app
            .world_mut()
            .query_filtered::<Entity, With<DebugHudText>>();
        assert_eq!(hud_query.iter(app.world()).count(), 1);

        let mut app_scene_entities = app
            .world_mut()
            .query_filtered::<Entity, With<AppSceneEntity>>();
        assert_eq!(app_scene_entities.iter(app.world()).count(), 7);
    }

    #[test]
    fn zoo_scene_groups_only_game_models_under_scene_root() {
        let mut app = App::new();
        app.init_resource::<Assets<Mesh>>()
            .init_resource::<Assets<StandardMaterial>>()
            .init_resource::<Assets<Image>>()
            .init_resource::<ZooPetDefaults>()
            .init_resource::<ZooSceneDefaults>()
            .add_systems(Startup, setup_zoo_scene);

        app.update();

        let mut root_query = app
            .world_mut()
            .query_filtered::<Entity, With<ZooSceneRoot>>();
        let root = root_query.single(app.world()).unwrap();
        assert_eq!(
            app.world().get::<Visibility>(root),
            Some(&Visibility::Inherited)
        );
        assert!(app.world().get::<InheritedVisibility>(root).is_some());
        assert_eq!(app.world().get::<Name>(root).unwrap().as_str(), "GameScene");
        let children = app.world().get::<Children>(root).unwrap();
        assert_eq!(children.len(), 5);

        let mut camera_query = app
            .world_mut()
            .query_filtered::<Entity, With<PrimarySceneCamera>>();
        assert_eq!(camera_query.iter(app.world()).count(), 0);

        let mut light_query = app.world_mut().query::<&DirectionalLight>();
        assert_eq!(light_query.iter(app.world()).count(), 0);

        let mut scene_entities = app
            .world_mut()
            .query_filtered::<Entity, With<ZooSceneEntity>>();
        assert_eq!(scene_entities.iter(app.world()).count(), 6);
    }

    #[test]
    fn zoo_scene_spawns_origin_marker_at_world_origin() {
        let mut app = App::new();
        app.init_resource::<Assets<Mesh>>()
            .init_resource::<Assets<StandardMaterial>>()
            .init_resource::<Assets<Image>>()
            .init_resource::<PrimaryCameraDefaults>()
            .init_resource::<ZooPetDefaults>()
            .init_resource::<ZooSceneDefaults>()
            .add_systems(Startup, setup_zoo_scene);

        app.update();

        let mut origin_query = app
            .world_mut()
            .query_filtered::<(&Name, &Transform), With<ZooSceneEntity>>();
        let (_, transform) = origin_query
            .iter(app.world())
            .find(|(name, _)| name.as_str() == "oriigin")
            .unwrap();

        assert_eq!(transform.translation, Vec3::ZERO);
        assert_eq!(transform.scale, Vec3::ONE);
    }

    #[test]
    fn zoo_scene_spawns_polar_bear_with_colormap_texture() {
        let mut app = App::new();
        app.init_resource::<Assets<Mesh>>()
            .init_resource::<Assets<StandardMaterial>>()
            .init_resource::<Assets<Image>>()
            .init_resource::<PrimaryCameraDefaults>()
            .init_resource::<ZooPetDefaults>()
            .init_resource::<ZooSceneDefaults>()
            .add_systems(Startup, setup_zoo_scene);

        app.update();

        let mut pet_query = app
            .world_mut()
            .query_filtered::<(&Name, &MeshMaterial3d<StandardMaterial>), With<ZooPet>>();
        let (name, material_handle) = pet_query.single(app.world()).unwrap();
        let materials = app.world().resource::<Assets<StandardMaterial>>();
        let material = materials.get(&material_handle.0).unwrap();

        assert_eq!(name.as_str(), "Zoo Pet Polar Bear");
        assert!(material.base_color_texture.is_some());
    }

    #[test]
    fn zoo_scene_spawns_platform_and_tree_with_colormap_textures() {
        let mut app = App::new();
        app.init_resource::<Assets<Mesh>>()
            .init_resource::<Assets<StandardMaterial>>()
            .init_resource::<Assets<Image>>()
            .init_resource::<PrimaryCameraDefaults>()
            .init_resource::<ZooPetDefaults>()
            .init_resource::<ZooSceneDefaults>()
            .add_systems(Startup, setup_zoo_scene);

        app.update();

        let mut model_query = app
            .world_mut()
            .query_filtered::<(&Name, &MeshMaterial3d<StandardMaterial>), With<ZooSceneEntity>>();
        let materials = app.world().resource::<Assets<StandardMaterial>>();
        let textured_names = model_query
            .iter(app.world())
            .filter_map(|(name, material_handle)| {
                let material = materials.get(&material_handle.0)?;
                material
                    .base_color_texture
                    .is_some()
                    .then_some(name.as_str().to_string())
            })
            .collect::<Vec<_>>();

        assert!(textured_names.contains(&"Zoo Platform".to_string()));
        assert!(textured_names.contains(&"Zoo Pine Tree".to_string()));
    }

    #[test]
    fn zoo_scene_spawns_rotating_star_at_requested_transform() {
        let mut app = App::new();
        app.init_resource::<Assets<Mesh>>()
            .init_resource::<Assets<StandardMaterial>>()
            .init_resource::<Assets<Image>>()
            .init_resource::<PrimaryCameraDefaults>()
            .init_resource::<ZooPetDefaults>()
            .init_resource::<ZooSceneDefaults>()
            .add_systems(Startup, setup_zoo_scene);

        app.update();

        let mut star_query = app
            .world_mut()
            .query_filtered::<(&Name, &Transform, &RotationComponent), With<ZooSceneEntity>>();
        let (_, transform, rotation) = star_query
            .iter(app.world())
            .find(|(name, _, _)| name.as_str() == "Zoo Star")
            .unwrap();

        assert_eq!(transform.translation, Vec3::new(0.0, 2.4, 1.6));
        assert_eq!(transform.scale, Vec3::splat(0.28));
        assert_eq!(rotation.radians_per_frame, STAR_ROTATION_PER_FRAME);
    }

    #[test]
    fn rotation_system_applies_radians_per_frame() {
        let mut app = App::new();
        app.add_systems(Update, rotation_system);
        app.world_mut().spawn((
            RotationComponent::new(STAR_ROTATION_PER_FRAME),
            Transform::default(),
        ));

        app.update();

        let mut query = app.world_mut().query::<&Transform>();
        let transform = query.single(app.world()).unwrap();
        let (_, yaw, _) = transform.rotation.to_euler(EulerRot::XYZ);
        assert!((yaw - STAR_ROTATION_PER_FRAME.y).abs() < f32::EPSILON);
    }

    #[test]
    fn model_browser_grid_transform_is_unrotated() {
        let transform = browser_grid_transform(0);

        assert_eq!(transform.rotation, Quat::IDENTITY);
        assert_eq!(transform.scale, Vec3::splat(MODEL_BROWSER_GRID_SCALE));
    }

    #[test]
    fn model_browser_showcase_transform_starts_unrotated() {
        let transform = browser_showcase_transform(-1.0);

        assert_eq!(transform.rotation, Quat::IDENTITY);
        assert_eq!(
            transform.translation,
            Vec3::new(0.0, -1.0, MODEL_BROWSER_SHOWCASE_Z)
        );
        assert_eq!(transform.scale, Vec3::splat(MODEL_BROWSER_SHOWCASE_SCALE));
    }

    #[test]
    fn model_browser_showcase_offsets_center_selected_models() {
        assert_eq!(
            browser_showcase_y_offset(0),
            -MODEL_BROWSER_ANIMAL_HALF_HEIGHTS[0] * MODEL_BROWSER_SHOWCASE_SCALE
        );
        assert!(browser_showcase_y_offset(2) < browser_showcase_y_offset(0));

        let transform = browser_showcase_transform(browser_showcase_y_offset(0));
        assert_eq!(
            transform.translation.y,
            -MODEL_BROWSER_ANIMAL_HALF_HEIGHTS[0] * MODEL_BROWSER_SHOWCASE_SCALE
        );
    }

    #[test]
    fn selected_model_browser_rotation_applies_requested_yaw() {
        let mut app = App::new();
        app.add_systems(Update, rotation_system);
        app.world_mut().spawn((
            RotationComponent::new(MODEL_BROWSER_SELECTED_ROTATION_PER_FRAME),
            Transform::default(),
        ));

        app.update();

        let mut query = app.world_mut().query::<&Transform>();
        let transform = query.single(app.world()).unwrap();
        let (_, yaw, _) = transform.rotation.to_euler(EulerRot::XYZ);
        assert!((yaw - 0.01).abs() < f32::EPSILON);
    }

    #[test]
    fn debug_hud_restart_key_is_not_toggle() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_systems(Startup, setup_debug_hud);

        app.update();

        let mut key_query = app.world_mut().query::<&DebugHudKeyText>();
        let restart_key = key_query
            .iter(app.world())
            .find(|key_text| key_text.key_code == KeyCode::KeyR)
            .unwrap();

        assert!(!restart_key.is_toggle);
    }

    #[test]
    fn debug_hud_h_key_is_toggle() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_systems(Startup, setup_debug_hud);

        app.update();

        let mut key_query = app.world_mut().query::<&DebugHudKeyText>();
        let hud_key = key_query
            .iter(app.world())
            .find(|key_text| key_text.key_code == KeyCode::KeyH)
            .unwrap();

        assert!(hud_key.is_toggle);
    }

    #[test]
    fn debug_hud_b_key_is_model_browser_toggle() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_systems(Startup, setup_debug_hud);

        app.update();

        let mut key_query = app.world_mut().query::<&DebugHudKeyText>();
        let browser_key = key_query
            .iter(app.world())
            .find(|key_text| key_text.key_code == KeyCode::KeyB)
            .unwrap();

        assert!(browser_key.is_toggle);
    }

    #[test]
    fn debug_hud_h_key_toggles_hot_reload_autorestart() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .init_resource::<ButtonInput<KeyCode>>()
            .init_resource::<DebugHudState>()
            .init_resource::<GameTicks>()
            .add_systems(Startup, setup_debug_hud)
            .add_systems(Update, (toggle_debug_hud_inputs, update_debug_hud));

        app.update();

        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyH);
        app.update();

        assert!(
            app.world()
                .resource::<DebugHudState>()
                .is_hot_reload_autorestart_enabled
        );

        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .reset(KeyCode::KeyH);
        app.update();
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyH);
        app.update();

        assert!(
            !app.world()
                .resource::<DebugHudState>()
                .is_hot_reload_autorestart_enabled
        );
    }

    #[test]
    fn restart_zoo_scene_reloads_scene_and_resets_ticks() {
        let mut app = App::new();
        app.init_resource::<Assets<Mesh>>()
            .init_resource::<Assets<StandardMaterial>>()
            .init_resource::<Assets<Image>>()
            .init_resource::<ButtonInput<KeyCode>>()
            .init_resource::<GameTicks>()
            .init_resource::<PrimaryCameraDefaults>()
            .init_resource::<ZooPetDefaults>()
            .init_resource::<ZooSceneDefaults>()
            .add_systems(Startup, (setup_app_scene, setup_zoo_scene).chain())
            .add_systems(Update, restart_zoo_scene);

        app.update();

        app.world_mut().resource_mut::<GameTicks>().0 = 42;
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyR);
        app.update();

        let mut root_query = app
            .world_mut()
            .query_filtered::<Entity, With<ZooSceneRoot>>();
        assert_eq!(root_query.iter(app.world()).count(), 1);
        assert_eq!(app.world().resource::<GameTicks>().0, 0);

        let mut camera_query = app
            .world_mut()
            .query_filtered::<Entity, With<PrimarySceneCamera>>();
        assert_eq!(camera_query.iter(app.world()).count(), 1);
    }

    #[test]
    fn b_key_switches_from_game_scene_to_model_browser_grid() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()))
            .init_resource::<Assets<Mesh>>()
            .init_resource::<Assets<StandardMaterial>>()
            .init_resource::<Assets<Image>>()
            .init_asset::<Scene>()
            .init_resource::<ButtonInput<KeyCode>>()
            .init_resource::<GameTicks>()
            .init_resource::<ActiveScene>()
            .init_resource::<ModelBrowserSelection>()
            .init_resource::<PrimaryCameraDefaults>()
            .init_resource::<ZooPetDefaults>()
            .init_resource::<ZooSceneDefaults>()
            .add_systems(Startup, (setup_app_scene, setup_zoo_scene).chain())
            .add_systems(Update, toggle_scene_browser);

        app.update();

        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyB);
        app.update();

        assert_eq!(
            *app.world().resource::<ActiveScene>(),
            ActiveScene::ModelBrowser
        );

        let mut zoo_root_query = app
            .world_mut()
            .query_filtered::<Entity, With<ZooSceneRoot>>();
        assert_eq!(zoo_root_query.iter(app.world()).count(), 0);

        let mut browser_root_query = app
            .world_mut()
            .query_filtered::<Entity, With<ModelBrowserSceneRoot>>();
        let browser_root = browser_root_query.single(app.world()).unwrap();
        assert_eq!(
            app.world().get::<Visibility>(browser_root),
            Some(&Visibility::Inherited)
        );
        assert!(
            app.world()
                .get::<InheritedVisibility>(browser_root)
                .is_some()
        );

        let mut no_indirect_camera_query = app
            .world_mut()
            .query_filtered::<Entity, (With<PrimarySceneCamera>, With<NoIndirectDrawing>)>();
        assert_eq!(no_indirect_camera_query.iter(app.world()).count(), 1);

        let mut app_root_query = app
            .world_mut()
            .query_filtered::<Entity, With<AppSceneRoot>>();
        assert_eq!(app_root_query.iter(app.world()).count(), 1);

        let mut browser_model_query = app
            .world_mut()
            .query_filtered::<Entity, With<BrowserAnimalModel>>();
        assert_eq!(
            browser_model_query.iter(app.world()).count(),
            MODEL_BROWSER_MODEL_COUNT
        );
    }
}
