use std::f32::consts::PI;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::math::{vec2, vec3};
use bevy::prelude::*;
use bevy::time::Stopwatch;
use bevy::window::{close_on_esc, PrimaryWindow};
// Window
const WW: f32 = 1200.0;
const WH: f32 = 900.0;

// Sprites
const SPRITE_SHEET_PATH: &str = "assets.png";
const SPRITE_SCALE_FACTOR: f32 = 3.0;
const TILE_W: usize = 16;
const TILE_H: usize = 16;
const SPRITE_SHEET_W: usize = 4;
const SPRITE_SHEET_H: usize = 4;
const BULLET_SPAWN_INTERVAL: f32 = 0.2;
const BULLET_SPEED: f32 = 10.0;
// Colors
const BG_COLOR: (u8, u8, u8) = (197, 204, 184);

// Resources
#[derive(Resource)]
struct GlobalTextureAtlasHandle(Option<Handle<TextureAtlasLayout>>);
#[derive(Resource)]
struct GlobalSpriteSheetHandle(Option<Handle<Image>>);
#[derive(Resource)]
struct CursorPosition(Option<Vec2>);
// Player
const PLAYER_SPEED: f32 = 2.0;
// Components
#[derive(Component)]
struct Player;

#[derive(Component)]
struct Gun;

#[derive(Component)]
struct GunTimer(Stopwatch);

#[derive(Component)]
struct Bullet;

#[derive(Component)]
struct BulletDirection(Vec3);

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
  #[default]
  Loading,
  GameInit,
  InGame,
}

fn main() {
  App::new()
    .init_state::<GameState>()
    .add_plugins(
      DefaultPlugins
        .set(ImagePlugin::default_nearest())
        .set(WindowPlugin {
          primary_window: Some(Window {
            resizable: true,
            focused: true,
            resolution: (WW, WH).into(),
            ..default()
          }),
          ..default()
        }),
    )
    .insert_resource(ClearColor(Color::rgb_u8(
      BG_COLOR.0, BG_COLOR.1, BG_COLOR.2,
    )))
    .insert_resource(Msaa::Off)
    .add_plugins(LogDiagnosticsPlugin::default())
    .add_plugins(FrameTimeDiagnosticsPlugin)
    // Custom resources
    .insert_resource(GlobalTextureAtlasHandle(None))
    .insert_resource(GlobalSpriteSheetHandle(None))
    .insert_resource(CursorPosition(None))
    // Systems
    .add_systems(OnEnter(GameState::Loading), load_assets)
    .add_systems(OnEnter(GameState::GameInit), (setup_camera, init_world))
    .add_systems(
      Update,
      (
        handle_player_input,
        update_gun_transform,
        update_cursor_position,
        handle_gun_input,
        update_bullets,
      )
        .run_if(in_state(GameState::InGame)),
    )
    .add_systems(Update, close_on_esc)
    .run();
}

fn setup_camera(mut commands: Commands) {
  commands.spawn(Camera2dBundle::default());
}

fn load_assets(
  mut texture_atlas: ResMut<GlobalTextureAtlasHandle>,
  mut image_handle: ResMut<GlobalSpriteSheetHandle>,
  asset_server: Res<AssetServer>,
  mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
  mut next_state: ResMut<NextState<GameState>>,
) {
  image_handle.0 = Some(asset_server.load(SPRITE_SHEET_PATH));
  let layout = TextureAtlasLayout::from_grid(
    Vec2::new(TILE_W as f32, TILE_H as f32),
    SPRITE_SHEET_W,
    SPRITE_SHEET_H,
    None,
    None,
  );
  texture_atlas.0 = Some(texture_atlas_layouts.add(layout));
  next_state.set(GameState::GameInit);
}
fn init_world(
  mut commands: Commands,
  texture_atlas: Res<GlobalTextureAtlasHandle>,
  image_handle: Res<GlobalSpriteSheetHandle>,
  mut next_state: ResMut<NextState<GameState>>,
) {
  commands.spawn((
    SpriteSheetBundle {
      texture: image_handle.0.clone().unwrap(),
      atlas: TextureAtlas {
        layout: texture_atlas.0.clone().unwrap(),
        index: 0,
      },
      transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
      ..default()
    },
    Player,
  ));
  commands.spawn((
    SpriteSheetBundle {
      texture: image_handle.0.clone().unwrap(),
      atlas: TextureAtlas {
        layout: texture_atlas.0.clone().unwrap(),
        index: 2,
      },
      transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
      ..default()
    },
    Gun,
    GunTimer(Stopwatch::new()),
  ));
  next_state.set(GameState::InGame);
}

fn handle_player_input(
  mut player_query: Query<&mut Transform, With<Player>>,
  keyboard_input: Res<ButtonInput<KeyCode>>,
) {
  if player_query.is_empty() {
    return;
  }
  let mut transform = player_query.single_mut();
  let a_key = keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft);
  let w_key = keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp);
  let s_key = keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown);
  let d_key = keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight);
  let mut delta = Vec2::ZERO;
  if a_key {
    delta.x -= 1.0;
  }
  if w_key {
    delta.y += 1.0;
  }
  if s_key {
    delta.y -= 1.0;
  }
  if d_key {
    delta.x += 1.0;
  }
  delta = delta.normalize_or_zero();
  transform.translation += vec3(delta.x, delta.y, 0.0) * PLAYER_SPEED;
}

fn update_gun_transform(
  cursor_position: Res<CursorPosition>,
  player_query: Query<&mut Transform, With<Player>>,
  mut gun_query: Query<&mut Transform, (With<Gun>, Without<Player>)>,
) {
  if player_query.is_empty() || gun_query.is_empty() {
    return;
  }
  let player_pos = player_query.single().translation.truncate();
  let cursor_pos = match cursor_position.0 {
    Some(cursor_pos) => cursor_pos,
    None => player_pos,
  };
  let mut gun_transform = gun_query.single_mut();

  let angle = (player_pos.y - cursor_pos.y).atan2(player_pos.x - cursor_pos.x) + PI;
  gun_transform.rotation = Quat::from_rotation_z(angle);

  let offset = 20.0;
  let new_gun_pos = vec2(
    player_pos.x + offset * angle.cos() - 5.0,
    player_pos.y + offset * angle.sin() - 15.0,
  );
  gun_transform.translation = vec3(new_gun_pos.x, new_gun_pos.y, gun_transform.translation.z);
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

fn handle_gun_input(
  mut commands: Commands,
  time: Res<Time>,
  texture_atlas: Res<GlobalTextureAtlasHandle>,
  image_handle: Res<GlobalSpriteSheetHandle>,
  mouse_button_input: Res<ButtonInput<MouseButton>>,
  mut gun_query: Query<(&Transform, &mut GunTimer), With<Gun>>,
) {
  if gun_query.is_empty() {
    return;
  }

  let (gun_transform, mut gun_timer) = gun_query.single_mut();
  let gun_pos = gun_transform.translation.truncate();
  gun_timer.0.tick(time.delta());

  if !mouse_button_input.pressed(MouseButton::Left) {
    return;
  }

  let bullet_dirction = gun_transform.local_x();
  if gun_timer.0.elapsed_secs() >= BULLET_SPAWN_INTERVAL {
    gun_timer.0.reset();
    commands.spawn((
      SpriteSheetBundle {
        texture: image_handle.0.clone().unwrap(),
        atlas: TextureAtlas {
          layout: texture_atlas.0.clone().unwrap(),
          index: 3,
        },
        transform: Transform::from_translation(vec3(gun_pos.x, gun_pos.y, 1.0))
          .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
        ..default()
      },
      Bullet,
      BulletDirection(*bullet_dirction),
    ));
  }
}

fn update_bullets(mut bullet_query: Query<(&mut Transform, &BulletDirection), With<Bullet>>) {
  if bullet_query.is_empty() {
    return;
  }
  for (mut transform, direction) in bullet_query.iter_mut() {
    transform.translation += vec3(direction.0.x, direction.0.y, 0.0) * BULLET_SPEED;
    transform.translation.z = 10.0;
  }
}
