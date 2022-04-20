use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::{movement::{Velocity, Accelleration}, tilemap::TileGrid};

pub struct CollisionsPlugin;

impl Plugin for CollisionsPlugin {
  fn build(&self, app:&mut App) {
      app
        .add_system(collisions.label("collisions").after("velocity").before("movement"));
  }
}

#[derive(Component, Inspectable)]
pub struct Collidable {
  pub radius: f32
}

fn collisions(
  mut collidables: Query<(&Transform, &mut Velocity, &mut Accelleration, &Collidable)>,
  tile_maps: Query<(&TileGrid, &Name)>,
  time: Res<Time>
) {
  for (position, mut velocity, mut accel, collidable) in collidables.iter_mut() {
    let mut shift = velocity.value * time.delta_seconds();
    //y collisions
    if velocity.value.y != 0. {
      let c = Vec2::new(position.translation.x, position.translation.y + shift.y);
      for (grid, name) in tile_maps.iter() {
        if name.starts_with("ground") && grid.in_radius(c, collidable.radius) {
          velocity.value.y = -velocity.value.y;
          accel.clear_y();
        }
      }
    }
    shift = velocity.value * time.delta_seconds();
    //x collisions
    if velocity.value.x != 0. {
      let c = Vec2::new(position.translation.x + shift.x, position.translation.y);
      for (grid, name) in tile_maps.iter() {
        if name.starts_with("ground") && grid.in_radius(c, collidable.radius) {
          velocity.value.x = 0.;
          accel.clear_x();
        }
      }
    }
  }
}