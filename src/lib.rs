pub mod animations;
pub mod camera;
pub mod constants;
pub mod enemy;
pub mod gui;
pub mod gun;
pub mod player;
pub mod resources;
pub mod state;
pub mod world;

use bevy::prelude::*;

pub fn out_of_bounds(pos: &Vec3, world_width: f32, world_h: f32) -> bool {
    if pos.x > world_width || pos.x < -world_width || pos.y > world_h || pos.y < -world_h {
        return true;
    }
    return false;
}
