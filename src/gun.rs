use crate::{
  player::Player,
  resources::{CursorPosition, GlobalTextureAtlas},
  state::GameState,
  BULLET_SPAWN_INTERVAL, BULLET_SPEED, SPRITE_SCALE_FACTOR,
};
use bevy::{
  math::{vec2, vec3},
  prelude::*,
  time::Stopwatch,
};
use std::f32::consts::PI;

#[derive(Component)]
pub struct Gun;

#[derive(Component)]
pub struct GunTimer(pub Stopwatch);

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
struct BulletDirection(Vec3);

pub struct GunPlugin;

impl Plugin for GunPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(
      Update,
      (update_gun_transform, handle_gun_input, update_bullets).run_if(in_state(GameState::InGame)),
    );
  }
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

fn handle_gun_input(
  mut commands: Commands,
  time: Res<Time>,
  texture_atlas: Res<GlobalTextureAtlas>,
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
        texture: texture_atlas.image.clone().unwrap(),
        atlas: TextureAtlas {
          layout: texture_atlas.layout.clone().unwrap(),
          index: 16,
        },
        transform: Transform::from_translation(vec3(gun_pos.x, gun_pos.y, 10.0))
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
  }
}

impl Default for GunTimer {
  fn default() -> Self {
    Self(Stopwatch::new())
  }
}
