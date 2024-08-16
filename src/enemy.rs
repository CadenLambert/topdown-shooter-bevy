use crate::animations::AnimationTimer;
use crate::constants::{
    ENEMY_HEALTH, ENEMY_SPAWN_RATE, ENEMY_SPEED, MAX_ENEMY_COUNT, SPRITE_SCALE_FACTOR, WORLD_H,
    WORLD_W,
};
use crate::player::Player;
use crate::resources::GameEntitySpriteAtlas;
use crate::state::GameState;
use bevy::{math::vec3, prelude::*};
use rand::Rng;

#[derive(Component)]
pub struct Enemy {
    pub health: f32,
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            health: ENEMY_HEALTH,
        }
    }
}

#[derive(Component)]
pub enum EnemyType {
    Grub = 1,
    Skele = 2,
    Gob = 3,
    Devil = 4,
    Demon = 5,
}

#[derive(Component, Default)]
pub enum EnemyState {
    Idle,
    #[default]
    Run,
}

impl EnemyType {
    pub fn get_random_enemy_type() -> Self {
        let mut rng = rand::thread_rng();
        return match rng.gen_range(0..5) {
            0 => Self::Grub,
            1 => Self::Skele,
            2 => Self::Gob,
            3 => Self::Devil,
            _ => Self::Demon,
        };
    }

    pub fn get_sprite_sheet_index(&self) -> usize {
        match self {
            EnemyType::Grub => 1,
            EnemyType::Skele => 2,
            EnemyType::Gob => 3,
            EnemyType::Devil => 4,
            EnemyType::Demon => 5,
        }
    }
}

#[derive(Resource)]
struct SpawnTimer(Timer);

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer(Timer::from_seconds(
            ENEMY_SPAWN_RATE,
            TimerMode::Once,
        )))
        .add_systems(
            Update,
            (spawn_enemies, approach_player, despawn_dead_enemies)
                .run_if(in_state(GameState::InGame)),
        );
    }
}

fn spawn_enemies(
    mut commands: Commands,
    game_entities: Res<GameEntitySpriteAtlas>,
    //global_sprite: Res<GlobalSpriteTextureHandle>,
    enemy_query: Query<&Transform, (With<Enemy>, Without<Player>)>,
    mut spawn_timer: ResMut<SpawnTimer>,
    time: Res<Time>,
) {
    spawn_timer.0.tick(time.delta());
    if spawn_timer.0.finished() {
        let num_enemies = enemy_query.iter().len();
        if num_enemies >= MAX_ENEMY_COUNT {
            return;
        }
        let enemies_to_spawn = (MAX_ENEMY_COUNT - num_enemies).min(10);
        let enemy_type_to_spawn = EnemyType::get_random_enemy_type();
        let mut rng = rand::thread_rng();
        for _ in 0..enemies_to_spawn {
            let x = rng.gen_range(-WORLD_W..WORLD_W);
            let y = rng.gen_range(-WORLD_H..WORLD_H);
            commands.spawn((
                SpriteBundle {
                    texture: game_entities.entity_sheets
                        [enemy_type_to_spawn.get_sprite_sheet_index()]
                    .clone()
                    .unwrap(),
                    transform: Transform::from_translation(vec3(x, y, 2.0))
                        .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                    ..default()
                },
                TextureAtlas {
                    layout: game_entities.atlas_layout.clone().unwrap(),
                    index: 0,
                },
                Enemy::default(),
                EnemyState::default(),
                AnimationTimer(Timer::from_seconds(0.125, TimerMode::Repeating)),
            ));
        }
        spawn_timer.0.reset();
    }
}

fn despawn_dead_enemies(mut commands: Commands, enemy_query: Query<(&Enemy, Entity), With<Enemy>>) {
    for (enemy, entity) in enemy_query.iter() {
        if enemy.health <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn approach_player(
    mut enemy_query: Query<&mut Transform, (With<Enemy>, Without<Player>)>,
    player_query: Query<&Transform, With<Player>>,
    time: Res<Time>,
) {
    if player_query.is_empty() {
        return;
    }
    let player_translation = player_query.single().translation.truncate();

    for mut enemy in &mut enemy_query {
        let enemy_translation = enemy.translation.truncate();
        let direction = (player_translation - enemy_translation).normalize()
            * ENEMY_SPEED
            * time.delta_seconds();
        enemy.translation += vec3(direction.x, direction.y, 0.0);
    }
}
