use bevy::{math::vec3, prelude::*};

use crate::{state::GameState, PLAYER_SPEED};

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(
      Update,
      (handle_player_input).run_if(in_state(GameState::InGame)),
    );
  }
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
