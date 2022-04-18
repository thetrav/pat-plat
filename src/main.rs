#![allow(clippy::redundant_field_names)]
use bevy::{prelude::*, render::camera::ScalingMode};

pub const CLEAR: Color = Color::rgb(0.1,0.1,0.1);
pub const RESOLUTION: f32 = 18.0 / 9.0;
pub const TILE_SIZE: f32 = 18.0;

mod player;
mod debug;
mod spritesheet;
mod tilemap;
mod movement;

use player::PlayerPlugin;
use debug::DebugPlugin;
use spritesheet::SpriteSheetPlugin;
use tilemap::TileMapPlugin;
use movement::PhysicsMovementPlugin;

fn main() {
    let height = 700.0;
    App::new()
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            width: height * RESOLUTION,
            height: height,
            title: "Pat's Platformer".to_string(),
            vsync: true,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_camera)
        .add_plugin(SpriteSheetPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(TileMapPlugin)
        .add_plugin(PhysicsMovementPlugin)
        .add_plugin(DebugPlugin)
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}


fn spawn_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();

    camera.orthographic_projection.top = 160.0;
    camera.orthographic_projection.bottom = -160.0;

    camera.orthographic_projection.left = -160.0 * RESOLUTION;
    camera.orthographic_projection.right = 160.0 * RESOLUTION;

    camera.orthographic_projection.scaling_mode = ScalingMode::None;

    commands.spawn_bundle(camera);
}


