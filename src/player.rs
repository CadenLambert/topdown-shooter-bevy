use crate::{constants::*, state::*};
use bevy::{math::vec3, prelude::*};

#[derive(Component)]
pub struct Player;

#[derive(Component, Default)]
pub enum PlayerState {
    #[default]
    Idle,
    Run,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            handle_player_input.run_if(in_state(GameState::InGame)),
        );
    }
}
fn handle_player_input(
    mut player_query: Query<(&mut Transform, &mut PlayerState), With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    if player_query.is_empty() {
        return;
    }

    let (mut transform, mut state) = player_query.single_mut();
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
        transform.translation += vec3(delta.x, delta.y, 0.0) * PLAYER_SPEED * time.delta_seconds();
        *state = PlayerState::Run;
    } else {
        *state = PlayerState::Idle;
    }
}
