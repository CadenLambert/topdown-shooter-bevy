use crate::{
    constants::*, out_of_bounds, player::Player, resources::*, state::*, world::GameEntity,
};
use bevy::{
    math::{vec2, vec3},
    prelude::*,
};
use std::f32::consts::PI;

#[derive(Component)]
pub struct GunCooldown(pub Timer);

#[derive(Component)]
pub struct Gun;

#[derive(Component)]
pub struct Bullet {
    velocity: Vec3,
    lifetime: Timer,
}

pub struct GunPlugin;

impl Plugin for GunPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_gun_transform, update_bullets, handle_gun_input)
                .run_if(in_state(GameState::InGame)),
        );
    }
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
    game_resource: Res<GameResourceSpriteAtlas>,
    //global_sprite: Res<GlobalSpriteTextureHandle>,
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
                texture: game_resource.sprite_sheet.clone().unwrap(),
                transform: Transform {
                    translation: gun_transform.translation,
                    rotation: gun_transform.rotation,
                    scale: Vec3::splat(SPRITE_SCALE_FACTOR),
                },
                ..default()
            },
            TextureAtlas {
                layout: game_resource.atlas_layout.clone().unwrap(),
                index: 1,
            },
            Bullet {
                velocity: vec3(gun_rotation.cos(), gun_rotation.sin(), 0.0).normalize()
                    * BULLET_SPEED,
                lifetime: Timer::from_seconds(5.0, TimerMode::Once),
            },
            GameEntity,
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
            bullet_transform.translation += bullet.velocity * time.delta_seconds();
        }
    }
}
