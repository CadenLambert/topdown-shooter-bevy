use crate::{
    animations::AnimationTimer,
    constants::*,
    gun::*,
    player::{Player, PlayerState},
    resources::*,
    state::*,
};
use bevy::{math::vec3, prelude::*};
use rand::Rng;

#[derive(Component)]
pub struct GameEntity;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::GameInit),
            (init_world, spawn_world_decoration),
        )
        .add_systems(OnExit(GameState::InGame), despawn_all_game_entities);
    }
}

fn init_world(
    mut commands: Commands,
    game_entities: Res<GameEntitySpriteAtlas>,
    game_resources: Res<GameResourceSpriteAtlas>,
    mut player_health: ResMut<PlayerHealth>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    commands.spawn((
        SpriteBundle {
            texture: game_entities.entity_sheets[0].clone().unwrap(), //global_sprite.sprite_sheet.clone().unwrap(),
            transform: Transform::from_translation(vec3(0.0, 0.0, 3.0))
                .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            ..default()
        },
        TextureAtlas {
            layout: game_entities.atlas_layout.clone().unwrap(), //global_sprite.texture_atlas.clone().unwrap(),
            index: 0,
        },
        Player,
        PlayerState::default(),
        AnimationTimer(Timer::from_seconds(0.125, TimerMode::Repeating)),
        GameEntity,
    ));

    commands.spawn((
        SpriteBundle {
            texture: game_resources.sprite_sheet.clone().unwrap(), //global_sprite.sprite_sheet.clone().unwrap(),
            transform: Transform::from_translation(vec3(0.0, 0.0, 3.0))
                .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            ..default()
        },
        TextureAtlas {
            layout: game_resources.atlas_layout.clone().unwrap(), //global_sprite.texture_atlas.clone().unwrap(),
            index: 0,
        },
        Gun,
        GunCooldown(Timer::from_seconds(GUN_FIRE_RATE, TimerMode::Once)),
        GameEntity,
    ));

    player_health.value = 100.0;

    next_state.set(GameState::InGame);
}

fn spawn_world_decoration(
    mut commands: Commands,
    game_decorations: Res<GameDecorationSpriteAtlas>,
    //global_sprite: Res<GlobalSpriteTextureHandle>, // texture_atlas: Res<GlobalTextureAtlasHandle>,
    // image_handle: Res<GlobalSpriteSheetHandle>,
) {
    let mut rng = rand::thread_rng();
    for _ in 0..NUM_WORLD_DECORATIONS {
        let x = rng.gen_range(-WORLD_W..WORLD_W);
        let y = rng.gen_range(-WORLD_H..WORLD_H);
        commands.spawn((
            SpriteBundle {
                texture: game_decorations.sprite_sheet.clone().unwrap(),
                transform: Transform::from_translation(vec3(x, y, 1.0))
                    .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                ..default()
            },
            TextureAtlas {
                layout: game_decorations.atlas_layout.clone().unwrap(),
                index: rng.gen_range(0..=1),
            },
            GameEntity,
        ));
    }
}

fn despawn_all_game_entities(
    mut commands: Commands,
    all_entities: Query<Entity, With<GameEntity>>,
) {
    for e in all_entities.iter() {
        commands.entity(e).despawn_recursive();
    }
}
