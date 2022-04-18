use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::{movement::Velocity, tilemap::TileGrid};

pub struct CollisionsPlugin;

impl Plugin for CollisionsPlugin {
  fn build(&self, app:&mut App) {
      app
        .add_system(collisions.label("collisions").after("movement"));
  }
}

#[derive(Component, Inspectable)]
pub struct Collidable {
  pub radius: f32
}

fn collisions(
  mut collidables: Query<(&mut Transform, &mut Velocity, &Collidable)>,
  tile_maps: Query<(&TileGrid, &Name)>,
  time: Res<Time>
) {
  for (mut position, velocity, collidable) in collidables.iter_mut() {
    let c = Vec2::new(position.translation.x, position.translation.y);
    for (grid, name) in tile_maps.iter() {
      //TODO: sample neighborhood, draw lines, test against lines and implement slide
      if name.starts_with("ground") && grid.in_radius(c, collidable.radius) {
        position.translation -= velocity.value.extend(0.) * time.delta_seconds();
      }
    }
  }
}