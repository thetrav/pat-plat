use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

pub struct PhysicalUniverse {
  pub friction: f32,
  pub gravity: f32,
  pub speed_of_light: f32,
  pub velocity_epsilon: f32
}

impl PhysicalUniverse {
  pub fn new() -> Self {
    PhysicalUniverse { 
      friction: 10., 
      gravity: 1000.,
      speed_of_light: 200., 
      velocity_epsilon: 10. 
    }
  }
}

#[derive(Component, Inspectable)]
pub struct Velocity {
  pub value: Vec2
}

impl Velocity {
  pub fn new() -> Self {
    Velocity { value: Vec2::new(0.,0.) }
  }
}

#[derive(Component, Inspectable)]
pub struct Accelleration {
  pub forces: Vec<(String, Vec2)>
}

impl Accelleration {
  pub fn new() -> Self {
    Accelleration { forces: vec![] }
  }

  pub fn value(&self) -> Vec2 {
    let mut v = Vec2::new(0.,0.);
    for (_, force) in &self.forces {
      v += *force;
    }
    return v;
  }

  pub fn set_force(&mut self, name: &str, value: Vec2) {
    let index = self.forces.iter().position(|(n, _)| n == name);
    match index {
      Some(i) => {
        self.forces[i].1.x = value.x;
        self.forces[i].1.y = value.y;
      },
      None => {
        self.forces.push((name.to_owned(), value));
      }
    }
  }

  pub fn get_force(&mut self, name: &str) -> Vec2 {
    for (n, force) in &self.forces {
      if n == name {
        return force.clone();
      }
    }
    let v = Vec2::new(0.,0.);
    self.forces.push((name.to_owned(), v));
    return v;
  }

  pub fn clear_x(&mut self) {
    let mut forces = vec![];
    for (n, force) in &self.forces {
      forces.push((n.to_owned(), Vec2::new(0., force.y)));
    }
    self.forces = forces;
  }

  pub fn clear_y(&mut self) {
    let mut forces = vec![];
    for (n, force) in &self.forces {
      forces.push((n.to_owned(), Vec2::new(force.x, 0.)));
    }
    self.forces = forces;
  }
}

pub struct PhysicsMovementPlugin;

impl Plugin for PhysicsMovementPlugin {
  fn build(&self, app:&mut App) {
      app
        .add_system(physics_velocity.label("velocity").after("input"))
        .add_system(movement.label("movement").after("velocity"))
        .add_startup_system(new_physical_universe);

  }
}

fn new_physical_universe(mut commands: Commands) {
  commands.insert_resource(PhysicalUniverse::new());
}

fn constrain(s:f32, m: f32) -> f32 {
  if s > m {
    return m;
  }
  if s < -m {
    return -m;
  }
  return s;
}

fn physics_velocity(
  mut query: Query<(&mut Velocity, &mut Accelleration)>,
  universe: Res<PhysicalUniverse>,
  time: Res<Time>
) {
  let seconds = time.delta_seconds();
  for (mut velocity, mut accell) in query.iter_mut() {
    
    let mut gravity = accell.get_force("GRAVITY");
    gravity.y -= universe.gravity * seconds;
    accell.set_force("GRAVITY", gravity);

    velocity.value += accell.value() * seconds;

    let friction =  velocity.value * universe.friction * seconds;
    velocity.value -= friction;

    if velocity.value.length_squared() < universe.velocity_epsilon {
      velocity.value.x = 0.;
      velocity.value.y = 0.;
    }

    velocity.value.x = constrain(velocity.value.x, universe.speed_of_light);
    velocity.value.y = constrain(velocity.value.y, universe.speed_of_light);
  }
}

fn movement(
  mut query: Query<(&mut Transform, &Velocity)>,
  time: Res<Time>
) {
  for (mut position, velocity) in query.iter_mut() {
    let shift = velocity.value * time.delta_seconds();
    position.translation += shift.extend(0.);
  }
}
