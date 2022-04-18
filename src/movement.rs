use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

pub struct PhysicalUniverse {
  pub friction: f32,
  pub speed_of_light: f32,
  pub velocity_epsilon: f32
}

impl PhysicalUniverse {
  pub fn new() -> Self {
    PhysicalUniverse { 
      friction: 10., 
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
  pub value: Vec2
}

impl Accelleration {
  pub fn new() -> Self {
    Accelleration { value: Vec2::new(0.,0.) }
  }
}

pub struct PhysicsMovementPlugin;

impl Plugin for PhysicsMovementPlugin {
  fn build(&self, app:&mut App) {
      app
        .add_system(physics_movement.label("movement").after("input"))
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

fn physics_movement(
  mut query: Query<(&mut Transform, &mut Velocity, &Accelleration)>,
  universe: Res<PhysicalUniverse>,
  time: Res<Time>
) {
  let seconds = time.delta_seconds();
  for (mut position, mut velocity, accell) in query.iter_mut() {
    velocity.value += accell.value * seconds;
    let friction =  velocity.value * universe.friction * seconds;
    velocity.value -= friction;
    if velocity.value.length_squared() < universe.velocity_epsilon {
      velocity.value.x = 0.;
      velocity.value.y = 0.;
    }
    velocity.value.x = constrain(velocity.value.x, universe.speed_of_light);
    velocity.value.y = constrain(velocity.value.y, universe.speed_of_light);
    
    let shift = velocity.value * seconds;

    position.translation += shift.extend(0.);
  }
}
