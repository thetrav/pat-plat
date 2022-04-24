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
  pub size: f32
}

enum Direction {
  Up,
  Down,
  Left, 
  Right,
  Still
}

fn vec_direction(v: Vec2) -> Direction {
  if v.x == 0. && v.y < 0. {
    Direction::Down
  } else if v.x == 0. && v.y > 0. {
    Direction::Up
  } else if v.y == 0. && v.x < 0. {
    Direction::Left
  } else if v.y == 0. && v.x > 0. {
    Direction::Right
  } else {
    Direction::Still
  }
}

fn ray_origins(center: Vec3, size: f32, motion: Vec2) -> Vec<Vec2> {
  let dir = vec_direction(motion);
  let hs = size/2.;
  let cx = center.x;
  let cy = center.y;
  match dir {
    Direction::Up => vec![
      Vec2::new(cx-hs, cy+hs),
      Vec2::new(cx+hs, cy+hs),
    ],
    Direction::Down => vec![
      Vec2::new(cx-hs, cy-hs),
      Vec2::new(cx+hs, cy-hs),
    ],
    Direction::Left => vec![
      Vec2::new(cx-hs, cy+hs),
      Vec2::new(cx-hs, cy-hs),
    ],
    Direction::Right => vec![
      Vec2::new(cx+hs, cy+hs),
      Vec2::new(cx+hs, cy-hs),
    ],
    Direction::Still => vec![],

  }
}

fn collisions(
  mut collidables: Query<(&Transform, &mut Velocity, &mut Accelleration, &Collidable)>,
  tile_maps: Query<(&TileGrid, &Name)>,
  time: Res<Time>
) {
  for (position, mut velocity, mut accel, collidable) in collidables.iter_mut() {
    let p = position.translation;
    let ds = time.delta_seconds();
    for (grid, name) in tile_maps.iter() {
      if name.starts_with("ground") {
        //y collisions
        if velocity.value.y != 0. {
          for ray_origin in ray_origins(p, collidable.size, Vec2::new(0., velocity.value.y)) {
            let hit = grid.cast_axis_ray(ray_origin, Vec2::new(0., velocity.value.y * ds));
            match hit {
              Some(_v) => {
                //TODO: calculate distance to actual intersection then subtract that from the velocity
                velocity.value.y = 0.;
                accel.clear_y();
              },
              None => {}
            }
          }
        }
     
        if velocity.value.x != 0. {
          for ray_origin in ray_origins(p, collidable.size, Vec2::new(velocity.value.x * ds, 0.)) {
            let hit = grid.cast_axis_ray(ray_origin, Vec2::new(velocity.value.x * ds, 0.));
            match hit {
              Some(_v) => {
                //TODO: calculate distance to actual intersection then subtract that from the velocity
                velocity.value.x = 0.;
                accel.clear_x();
              },
              None => {}
            }
          }
        }
      }
    }
  }
}