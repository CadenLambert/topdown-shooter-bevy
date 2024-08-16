use crate::{camera::*, constants::*, state::*};
use bevy::{prelude::*, window::PrimaryWindow};

// Resources
#[derive(Resource)]
pub struct GlobalSpriteTextureHandle {
    pub texture_atlas: Option<Handle<TextureAtlasLayout>>,
    pub sprite_sheet: Option<Handle<Image>>,
}

#[derive(Resource)]
pub struct GameResourceSpriteAtlas {
    pub atlas_layout: Option<Handle<TextureAtlasLayout>>,
    pub sprite_sheet: Option<Handle<Image>>,
}

#[derive(Resource)]
pub struct GameEntitySpriteAtlas {
    pub atlas_layout: Option<Handle<TextureAtlasLayout>>,
    pub entity_sheets: Vec<Option<Handle<Image>>>,
}

#[derive(Resource)]
pub struct GameDecorationSpriteAtlas {
    pub atlas_layout: Option<Handle<TextureAtlasLayout>>,
    pub sprite_sheet: Option<Handle<Image>>,
}

#[derive(Resource)]
pub struct CursorPos(pub Option<Vec2>);

#[derive(Resource)]
pub struct PlayerHealth {
    pub value: f32,
}

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::srgb_u8(
            BG_COLOR.0, BG_COLOR.1, BG_COLOR.2,
        )))
        .insert_resource(Msaa::Off)
        .insert_resource(GameResourceSpriteAtlas {
            atlas_layout: None,
            sprite_sheet: None,
        })
        .insert_resource(GameDecorationSpriteAtlas {
            atlas_layout: None,
            sprite_sheet: None,
        })
        .insert_resource(GameEntitySpriteAtlas {
            atlas_layout: None,
            entity_sheets: vec![None; 6],
        })
        .insert_resource(CursorPos(None))
        .insert_resource(PlayerHealth { value: 100.0 })
        .add_systems(OnEnter(GameState::Loading), load_assets)
        .add_systems(
            Update,
            update_cursor_position.run_if(in_state(GameState::InGame)),
        );
    }
}

fn load_assets(
    //mut global_sprite: ResMut<GlobalSpriteTextureHandle>,
    mut game_entity: ResMut<GameEntitySpriteAtlas>,
    mut game_resource: ResMut<GameResourceSpriteAtlas>,
    mut game_decoration: ResMut<GameDecorationSpriteAtlas>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let common_entity_layout = TextureAtlasLayout::from_grid(
        UVec2::splat(SPRITE_TILE_SIZE),
        8,
        1,
        Some(UVec2::splat(0)),
        Some(UVec2::splat(1)),
    );
    game_entity.atlas_layout = Some(texture_atlas_layouts.add(common_entity_layout));
    game_entity.entity_sheets[0] = Some(asset_server.load("player_sheet.png"));
    game_entity.entity_sheets[1] = Some(asset_server.load("grub_sheet.png"));
    game_entity.entity_sheets[2] = Some(asset_server.load("skele_sheet.png"));
    game_entity.entity_sheets[3] = Some(asset_server.load("gob_sheet.png"));
    game_entity.entity_sheets[4] = Some(asset_server.load("devil_sheet.png"));
    game_entity.entity_sheets[5] = Some(asset_server.load("demon_sheet.png"));

    let resource_layout = TextureAtlasLayout::from_grid(
        UVec2::splat(SPRITE_TILE_SIZE),
        6,
        1,
        None,
        Some(UVec2::splat(1)),
    );
    game_resource.atlas_layout = Some(texture_atlas_layouts.add(resource_layout));
    game_resource.sprite_sheet = Some(asset_server.load("resource_sheet.png"));

    let decoration_layout = TextureAtlasLayout::from_grid(
        UVec2::splat(SPRITE_TILE_SIZE),
        2,
        1,
        Some(UVec2::new(1, 0)),
        Some(UVec2::splat(1)),
    );
    game_decoration.atlas_layout = Some(texture_atlas_layouts.add(decoration_layout));
    game_decoration.sprite_sheet = Some(asset_server.load("decoration_sheet.png"));

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
