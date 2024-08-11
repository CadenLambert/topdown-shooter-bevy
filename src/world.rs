use crate::{constants::*, gun::*, player::Player, resources::*, state::*};
use bevy::{math::vec3, prelude::*};
use rand::Rng;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::GameInit),
            (init_world, spawn_world_decoration),
        );
    }
}

fn init_world(
    mut commands: Commands,
    global_sprite: Res<GlobalSpriteTextureHandle>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    commands.spawn((
        SpriteBundle {
            texture: global_sprite.sprite_sheet.clone().unwrap(),
            transform: Transform::from_translation(vec3(0.0, 0.0, 3.0))
                .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            ..default()
        },
        TextureAtlas {
            layout: global_sprite.texture_atlas.clone().unwrap(),
            index: 0,
        },
        Player,
    ));

    commands.spawn((
        SpriteBundle {
            texture: global_sprite.sprite_sheet.clone().unwrap(),
            transform: Transform::from_translation(vec3(0.0, 0.0, 3.0))
                .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            ..default()
        },
        TextureAtlas {
            layout: global_sprite.texture_atlas.clone().unwrap(),
            index: 2,
        },
        Gun,
        GunCooldown(Timer::from_seconds(GUN_FIRE_RATE, TimerMode::Once)),
    ));

    next_state.set(GameState::InGame);
}

fn spawn_world_decoration(
    mut commands: Commands,
    global_sprite: Res<GlobalSpriteTextureHandle>, // texture_atlas: Res<GlobalTextureAtlasHandle>,
                                                   // image_handle: Res<GlobalSpriteSheetHandle>,
) {
    let mut rng = rand::thread_rng();
    for _ in 0..NUM_WORLD_DECORATIONS {
        let x = rng.gen_range(-WORLD_W..WORLD_W);
        let y = rng.gen_range(-WORLD_H..WORLD_H);

        let z = rng.gen_range(0.0..3.0);
        commands.spawn((
            SpriteBundle {
                texture: global_sprite.sprite_sheet.clone().unwrap(),
                transform: Transform::from_translation(vec3(x, y, z))
                    .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                ..default()
            },
            TextureAtlas {
                layout: global_sprite.texture_atlas.clone().unwrap(),
                index: rng.gen_range(4..=5),
            },
        ));
    }
}
