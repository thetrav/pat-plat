use bevy::prelude::*;
use bevy_inspector_egui::{WorldInspectorPlugin, RegisterInspectable};

use crate::{player::Player, movement::{Velocity, Accelleration}, collisions::Collidable};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app.add_plugin(WorldInspectorPlugin::new())
                .register_inspectable::<Player>()
                .register_inspectable::<Velocity>()
                .register_inspectable::<Accelleration>()
                .register_inspectable::<Collidable>();
        }
    }
}