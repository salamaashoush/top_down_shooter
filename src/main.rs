use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::close_on_esc;

use top_down_shooter::*;

use camera::CameraPlugin;
use gun::GunPlugin;
use player::PlayerPlugin;
use resources::ResourcesPlugin;
use state::GameState;
use world::WorldPlugin;

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
    .add_plugins(ResourcesPlugin)
    .add_plugins(CameraPlugin)
    .add_plugins(PlayerPlugin)
    .add_plugins(GunPlugin)
    .add_plugins(WorldPlugin)
    .add_systems(Update, close_on_esc)
    .run();
}
