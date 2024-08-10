use std::f32::consts::PI;

use bevy::{
    math::{vec2, vec3},
    prelude::*,
    window::PrimaryWindow,
};
use bevy_pancam::{PanCam, PanCamPlugin};
use rand::Rng;

pub const SPRITE_TILE_SIZE: u32 = 16;
pub const SPRITE_SCALE_FACTOR: f32 = 3.0;
pub const WW: f32 = 1200.0;
pub const WH: f32 = 700.0;

pub const PLAYER_SPEED: f32 = 2.0;
pub const BULLET_SPEED: f32 = 10.0;
pub const GUN_FIRE_RATE: f32 = 0.125;

pub const NUM_WORLD_DECORATIONS: usize = 1000;
pub const WORLD_W: f32 = 3000.0;
pub const WORLD_H: f32 = 2500.0;

// Colors
pub const BG_COLOR: (u8, u8, u8) = (197, 204, 184);

// Resources
#[derive(Resource)]
pub struct GlobalTextureAtlasHandle(Option<Handle<TextureAtlasLayout>>);
#[derive(Resource)]
pub struct GlobalSpriteSheetHandle(Option<Handle<Image>>);
#[derive(Resource)]
pub struct CursorPos(Option<Vec2>);

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Loading,
    GameInit,
    InGame,
}

#[derive(Resource)]
pub struct PlayerHealth {
    value: u32,
}

#[derive(Component)]
pub struct GunCooldown(Timer);

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Gun;

#[derive(Component)]
pub struct Bullet {
    velocity: Vec3,
    lifetime: Timer,
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resizable: true,
                        focused: true,
                        resolution: (WW, WH).into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .init_state::<GameState>() //Games keeps crashing if this comes before adding default plugins
        .insert_resource(ClearColor(Color::srgb_u8(
            BG_COLOR.0, BG_COLOR.1, BG_COLOR.2,
        )))
        .insert_resource(Msaa::Off)
        // External Plugins
        .add_plugins(PanCamPlugin::default())
        // Custom Resources
        .insert_resource(GlobalTextureAtlasHandle(None))
        .insert_resource(GlobalSpriteSheetHandle(None))
        .insert_resource(CursorPos(None))
        .insert_resource(PlayerHealth { value: 100 })
        // Systems
        .add_systems(OnEnter(GameState::Loading), load_assets)
        .add_systems(
            OnEnter(GameState::GameInit),
            (setup_camera, init_world, spawn_world_decoration),
        )
        .add_systems(
            Update,
            (
                handle_player_input,
                camera_follow_player,
                update_gun_transform,
                update_cursor_position,
                handle_gun_input,
                update_bullets,
            )
                .chain()
                .run_if(in_state(GameState::InGame)),
        )
        .run();
}

fn load_assets(
    mut texture_atlas: ResMut<GlobalTextureAtlasHandle>,
    mut image_handle: ResMut<GlobalSpriteSheetHandle>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    image_handle.0 = Some(asset_server.load("assets.png"));
    let layout = TextureAtlasLayout::from_grid(
        UVec2::splat(SPRITE_TILE_SIZE),
        4,
        4,
        Some(UVec2::splat(1)),
        None,
    );
    texture_atlas.0 = Some(texture_atlas_layouts.add(layout));

    next_state.set(GameState::GameInit);
}

// #[derive(Component)]
// struct AnimationIndices {
//     first: usize,
//     last: usize,
// }

// #[derive(Component, Deref, DerefMut)]
// struct AnimationTimer(Timer);

// fn animate_sprite(
//     time: Res<Time>,
//     mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
// ) {
//     for (indices, mut timer, mut atlas) in &mut query {
//         timer.tick(time.delta());
//         if timer.just_finished() {
//             atlas.index = if atlas.index == indices.last {
//                 indices.first
//             } else {
//                 atlas.index + 1
//             };
//         }
//     }
// }

fn setup_camera(mut commands: Commands) {
    commands
        .spawn((Camera2dBundle::default(), MainCamera))
        .insert(PanCam {
            grab_buttons: Vec::new(),
            min_scale: 1.0,
            max_scale: 6.0,
            ..default()
        });
}

fn camera_follow_player(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    if camera_query.is_empty() || player_query.is_empty() {
        return;
    }

    let mut camera_transform = camera_query.single_mut();
    let player_transform = player_query.single().translation;

    let (x, y) = (player_transform.x, player_transform.y);

    camera_transform.translation = camera_transform.translation.lerp(vec3(x, y, 0.0), 1.0);
}

fn init_world(
    mut commands: Commands,
    texture_atlas: Res<GlobalTextureAtlasHandle>,
    image_handle: Res<GlobalSpriteSheetHandle>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    commands.spawn((
        SpriteBundle {
            texture: image_handle.0.clone().unwrap(),
            transform: Transform::from_translation(vec3(0.0, 0.0, 3.0))
                .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            ..default()
        },
        TextureAtlas {
            layout: texture_atlas.0.clone().unwrap(),
            index: 0,
        },
        Player,
    ));

    commands.spawn((
        SpriteBundle {
            texture: image_handle.0.clone().unwrap(),
            transform: Transform::from_translation(vec3(0.0, 0.0, 3.0))
                .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            ..default()
        },
        TextureAtlas {
            layout: texture_atlas.0.clone().unwrap(),
            index: 2,
        },
        Gun,
        GunCooldown(Timer::from_seconds(GUN_FIRE_RATE, TimerMode::Once)),
    ));

    next_state.set(GameState::InGame);
}

fn handle_player_input(
    mut player_query: Query<&mut Transform, With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if player_query.is_empty() {
        return;
    }

    let mut transform = player_query.single_mut();
    let up = keyboard_input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]);
    let down = keyboard_input.any_pressed([KeyCode::KeyR, KeyCode::ArrowDown]);
    let left = keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]);
    let right = keyboard_input.any_pressed([KeyCode::KeyS, KeyCode::ArrowRight]);

    let mut delta = Vec2::ZERO;
    if up && transform.translation.y < WORLD_H {
        delta.y += 1.0;
    }
    if down && transform.translation.y > -WORLD_H {
        delta.y -= 1.0;
    }
    if left && transform.translation.x > -WORLD_W {
        delta.x -= 1.0;
    }
    if right && transform.translation.x < WORLD_W {
        delta.x += 1.0;
    }
    delta = delta.normalize();

    if delta.is_finite() {
        transform.translation += vec3(delta.x, delta.y, 0.0) * PLAYER_SPEED;
    }
}

fn update_cursor_position(
    mut cursor_pos: ResMut<CursorPos>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    if window_query.is_empty() || camera_query.is_empty() {
        cursor_pos.0 = None;
    }

    let (camera, camera_transform) = camera_query.single();
    let window = window_query.single();
    cursor_pos.0 = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate());
}

fn update_gun_transform(
    cursor_pos: Res<CursorPos>,
    player_query: Query<&Transform, With<Player>>,
    mut gun_query: Query<&mut Transform, (With<Gun>, Without<Player>)>,
) {
    if player_query.is_empty() || gun_query.is_empty() {
        return;
    }

    let player_position = player_query.single().translation.truncate();
    let cursor_position = match cursor_pos.0 {
        Some(pos) => pos,
        None => player_position,
    };
    let mut gun_transform = gun_query.single_mut();

    let angle = (cursor_position.y - player_position.y)
        .atan2(cursor_position.x - player_position.x)
        - (PI / 2.0);
    gun_transform.rotation = Quat::from_rotation_z(angle);

    let offset = 50.0;
    let new_gun_pos = vec2(
        player_position.x + offset * angle.sin() * -1.0,
        player_position.y + offset * angle.cos(),
    );
    gun_transform.translation = vec3(new_gun_pos.x, new_gun_pos.y, gun_transform.translation.z);
}

fn handle_gun_input(
    mut commands: Commands,
    texture_atlas: Res<GlobalTextureAtlasHandle>,
    image_handle: Res<GlobalSpriteSheetHandle>,
    mut gun_query: Query<(&Transform, &mut GunCooldown), (With<Gun>, Without<Player>)>,
    time: Res<Time>,
    mouse_input: Res<ButtonInput<MouseButton>>,
) {
    if gun_query.is_empty() {
        return;
    }
    let (gun_transform, mut gun_timer) = gun_query.single_mut();
    gun_timer.0.tick(time.delta());
    if mouse_input.just_pressed(MouseButton::Left)
        || (mouse_input.pressed(MouseButton::Left) && gun_timer.0.finished())
    {
        gun_timer.0.reset();
        let gun_rotation = gun_transform.rotation.to_euler(EulerRot::XYZ).2 + (PI / 2.0);

        commands.spawn((
            SpriteBundle {
                texture: image_handle.0.clone().unwrap(),
                transform: Transform {
                    translation: gun_transform.translation,
                    rotation: gun_transform.rotation,
                    scale: Vec3::splat(SPRITE_SCALE_FACTOR),
                },
                ..default()
            },
            TextureAtlas {
                layout: texture_atlas.0.clone().unwrap(),
                index: 3,
            },
            Bullet {
                velocity: vec3(gun_rotation.cos(), gun_rotation.sin(), 0.0).normalize()
                    * BULLET_SPEED,
                lifetime: Timer::from_seconds(5.0, TimerMode::Once),
            },
        ));
    }
}

fn update_bullets(
    mut commands: Commands,
    time: Res<Time>,
    mut bullet_query: Query<(Entity, &mut Transform, &mut Bullet)>,
) {
    for (bullet_entity, mut bullet_transform, mut bullet) in &mut bullet_query {
        bullet.lifetime.tick(time.delta());
        let bullet_pos = bullet_transform.translation;
        if bullet.lifetime.finished() || out_of_bounds(&bullet_pos, WORLD_W, WORLD_H) {
            commands.entity(bullet_entity).despawn();
        } else {
            bullet_transform.translation += bullet.velocity;
        }
    }
}

fn out_of_bounds(pos: &Vec3, world_width: f32, world_h: f32) -> bool {
    if pos.x > world_width || pos.x < -world_width || pos.y > world_h || pos.y < -world_h {
        return true;
    }
    return false;
}

fn spawn_world_decoration(
    mut commands: Commands,
    texture_atlas: Res<GlobalTextureAtlasHandle>,
    image_handle: Res<GlobalSpriteSheetHandle>,
) {
    let mut rng = rand::thread_rng();
    for _ in 0..NUM_WORLD_DECORATIONS {
        let x = rng.gen_range(-WORLD_W..WORLD_W);
        let y = rng.gen_range(-WORLD_H..WORLD_H);

        let z = rng.gen_range(0.0..3.0);
        commands.spawn((
            SpriteBundle {
                texture: image_handle.0.clone().unwrap(),
                transform: Transform::from_translation(vec3(x, y, z))
                    .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                ..default()
            },
            TextureAtlas {
                layout: texture_atlas.0.clone().unwrap(),
                index: rng.gen_range(4..=5),
            },
        ));
    }
}
