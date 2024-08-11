use crate::{camera::*, constants::*, state::*};
use bevy::{prelude::*, window::PrimaryWindow};

// Resources
#[derive(Resource)]
pub struct GlobalSpriteTextureHandle {
    pub texture_atlas: Option<Handle<TextureAtlasLayout>>,
    pub sprite_sheet: Option<Handle<Image>>,
}

#[derive(Resource)]
pub struct GlobalTextureAtlasHandle();
#[derive(Resource)]
pub struct GlobalSpriteSheetHandle(Option<Handle<Image>>);
#[derive(Resource)]
pub struct CursorPos(pub Option<Vec2>);

#[derive(Resource)]
pub struct PlayerHealth {
    value: u32,
}

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::srgb_u8(
            BG_COLOR.0, BG_COLOR.1, BG_COLOR.2,
        )))
        .insert_resource(Msaa::Off)
        // Custom Resources
        .insert_resource(GlobalSpriteTextureHandle {
            texture_atlas: None,
            sprite_sheet: None,
        })
        .insert_resource(CursorPos(None))
        .insert_resource(PlayerHealth { value: 100 })
        .add_systems(OnEnter(GameState::Loading), load_assets)
        .add_systems(
            Update,
            update_cursor_position.run_if(in_state(GameState::InGame)),
        );
    }
}

fn load_assets(
    mut global_sprite: ResMut<GlobalSpriteTextureHandle>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    global_sprite.sprite_sheet = Some(asset_server.load("assets.png"));
    let layout = TextureAtlasLayout::from_grid(
        UVec2::splat(SPRITE_TILE_SIZE),
        4,
        4,
        Some(UVec2::splat(1)),
        None,
    );
    global_sprite.texture_atlas = Some(texture_atlas_layouts.add(layout));

    next_state.set(GameState::GameInit);
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

impl Default for GlobalSpriteTextureHandle {
    fn default() -> Self {
        Self {
            texture_atlas: None,
            sprite_sheet: None,
        }
    }
}
