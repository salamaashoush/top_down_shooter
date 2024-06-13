use bevy::{prelude::*, window::PrimaryWindow};

use crate::{state::GameState, SPRITE_SHEET_H, SPRITE_SHEET_PATH, SPRITE_SHEET_W, TILE_H, TILE_W};

// Resources
#[derive(Resource)]
pub struct GlobalTextureAtlas {
  pub layout: Option<Handle<TextureAtlasLayout>>,
  pub image: Option<Handle<Image>>,
}

#[derive(Resource)]
pub struct CursorPosition(pub Option<Vec2>);

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(GlobalTextureAtlas::default())
      .insert_resource(CursorPosition(None))
      .add_systems(OnEnter(GameState::Loading), load_assets)
      .add_systems(
        Update,
        (update_cursor_position).run_if(in_state(GameState::InGame)),
      );
  }
}

impl Default for GlobalTextureAtlas {
  fn default() -> Self {
    Self {
      layout: None,
      image: None,
    }
  }
}

fn load_assets(
  mut texture_atlas: ResMut<GlobalTextureAtlas>,
  asset_server: Res<AssetServer>,
  mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
  mut next_state: ResMut<NextState<GameState>>,
) {
  texture_atlas.image = Some(asset_server.load(SPRITE_SHEET_PATH));
  let layout = TextureAtlasLayout::from_grid(
    Vec2::new(TILE_W as f32, TILE_H as f32),
    SPRITE_SHEET_W,
    SPRITE_SHEET_H,
    None,
    None,
  );
  texture_atlas.layout = Some(texture_atlas_layouts.add(layout));
  next_state.set(GameState::GameInit);
}

fn update_cursor_position(
  window_query: Query<&Window, With<PrimaryWindow>>,
  camera_query: Query<(&Camera, &GlobalTransform), With<Camera>>,
  mut cursor_position: ResMut<CursorPosition>,
) {
  if window_query.is_empty() || camera_query.is_empty() {
    cursor_position.0 = None;
  }

  let (camera, camera_transform) = camera_query.single();
  let window = window_query.single();
  cursor_position.0 = window
    .cursor_position()
    .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
    .map(|ray| ray.origin.truncate());
}
