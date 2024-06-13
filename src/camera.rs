use bevy::{math::vec3, prelude::*};
use bevy_pancam::{PanCam, PanCamPlugin};

use crate::{player::Player, state::GameState};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugins(PanCamPlugin::default())
      .add_systems(OnEnter(GameState::GameInit), setup_camera)
      .add_systems(
        Update,
        (camera_follow_player).run_if(in_state(GameState::InGame)),
      );
  }
}

fn setup_camera(mut commands: Commands) {
  commands
    .spawn(Camera2dBundle::default())
    .insert(PanCam::default());
}

fn camera_follow_player(
  player_query: Query<&Transform, With<Player>>,
  mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
  if player_query.is_empty() || camera_query.is_empty() {
    return;
  }
  let player_pos = player_query.single().translation.truncate();
  let mut camera_transform = camera_query.single_mut();
  camera_transform.translation = camera_transform
    .translation
    .lerp(vec3(player_pos.x, player_pos.y, 10.0), 0.1);
}
