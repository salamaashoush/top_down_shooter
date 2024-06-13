use bevy::{math::vec3, prelude::*};
use rand::Rng;

use crate::{
  gun::{Gun, GunTimer},
  player::Player,
  resources::GlobalTextureAtlas,
  state::GameState,
  SPRITE_SCALE_FACTOR, WORLD_DECORATION_COUNT, WORLD_H, WORLD_W,
};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(
      OnEnter(GameState::GameInit),
      (init_world, spawn_world_decoration),
    );
  }
}

fn spawn_world_decoration(mut commands: Commands, texture_atlas: Res<GlobalTextureAtlas>) {
  let mut rng = rand::thread_rng();
  for _ in 0..WORLD_DECORATION_COUNT {
    let x = rng.gen_range(-WORLD_W..WORLD_W);
    let y = rng.gen_range(-WORLD_H..WORLD_H);
    commands.spawn((
      SpriteSheetBundle {
        texture: texture_atlas.image.clone().unwrap(),
        atlas: TextureAtlas {
          layout: texture_atlas.layout.clone().unwrap(),
          index: rng.gen_range(24..=25),
        },
        transform: Transform::from_translation(vec3(x, y, 0.0))
          .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
        ..default()
      },
      // WorldDecoration,
    ));
  }
}

fn init_world(
  mut commands: Commands,
  texture_atlas: Res<GlobalTextureAtlas>,
  mut next_state: ResMut<NextState<GameState>>,
) {
  commands.spawn((
    SpriteSheetBundle {
      texture: texture_atlas.image.clone().unwrap(),
      atlas: TextureAtlas {
        layout: texture_atlas.layout.clone().unwrap(),
        index: 0,
      },
      transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR))
        .with_translation(Vec3::new(0.0, 0.0, 10.0)),
      ..default()
    },
    Player,
  ));
  commands.spawn((
    SpriteSheetBundle {
      texture: texture_atlas.image.clone().unwrap(),
      atlas: TextureAtlas {
        layout: texture_atlas.layout.clone().unwrap(),
        index: 17,
      },
      transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR))
        .with_translation(Vec3::new(0.0, 0.0, 15.0)),
      ..default()
    },
    Gun,
    GunTimer::default(),
  ));
  next_state.set(GameState::InGame);
}
