use crate::{
    enemy::{Enemy, EnemyState},
    gun::Gun,
    player::{Player, PlayerState},
    resources::CursorPos,
    state::GameState,
};
use bevy::prelude::*;

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                animation_timer_tick,
                animate_player,
                flip_gun_sprite_x,
                animate_enemy,
                flip_player_sprite_x,
                flip_enemy_sprite_x,
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}

fn animation_timer_tick(
    time: Res<Time>,
    mut query: Query<&mut AnimationTimer, With<AnimationTimer>>,
) {
    for mut timer in query.iter_mut() {
        timer.tick(time.delta());
    }
}

fn animate_player(
    mut player_query: Query<(&mut TextureAtlas, &PlayerState, &AnimationTimer), With<Player>>,
) {
    if player_query.is_empty() {
        return;
    }

    let (mut atlas, state, timer) = player_query.single_mut();

    if timer.just_finished() {
        let base_sprite_index = match state {
            PlayerState::Idle => 0,
            PlayerState::Run => 4,
        };
        atlas.index = base_sprite_index + ((atlas.index + 1) % 4);
    }
}

fn animate_enemy(
    mut enemy_query: Query<(&mut TextureAtlas, &EnemyState, &AnimationTimer), With<Enemy>>,
) {
    for (mut atlas, state, timer) in enemy_query.iter_mut() {
        if timer.just_finished() {
            let base_sprite_index = match state {
                EnemyState::Idle => 0,
                EnemyState::Run => 4,
            };
            atlas.index = base_sprite_index + ((atlas.index + 1) % 4);
        }
    }
}

fn flip_player_sprite_x(
    cursor_pos: Res<CursorPos>,
    mut player_query: Query<(&mut Sprite, &Transform), With<Player>>,
) {
    if player_query.is_empty() {
        return;
    }

    let (mut sprite, transform) = player_query.single_mut();
    if let Some(cursor_pos) = cursor_pos.0 {
        sprite.flip_x = cursor_pos.x <= transform.translation.x;
    }
}

fn flip_enemy_sprite_x(
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<(&mut Sprite, &Transform), With<Enemy>>,
) {
    if player_query.is_empty() {
        return;
    }
    let player_pos = player_query.single().translation;
    for (mut sprite, transform) in enemy_query.iter_mut() {
        sprite.flip_x = transform.translation.x >= player_pos.x;
    }
}

fn flip_gun_sprite_x(
    cursor_pos: Res<CursorPos>,
    mut gun_query: Query<(&mut Sprite, &Transform), With<Gun>>,
) {
    if gun_query.is_empty() {
        return;
    }

    let (mut sprite, transform) = gun_query.single_mut();
    if let Some(cursor_pos) = cursor_pos.0 {
        sprite.flip_x = cursor_pos.x <= transform.translation.x;
    }
}
