use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::{
    spritesheet::{CharacterAtlas, spawn_sprite, CHARACTER_TILE_SIZE}, 
    TILE_SIZE, 
    movement::{Accelleration, Velocity}, collisions::Collidable};

pub struct PlayerPlugin;

#[derive(Default, Component, Inspectable)]
pub struct Player {
    speed: f32,
    up: bool,
    down: bool,
    left: bool,
    right: bool,
    stick_pos: Vec2
}

impl Plugin for PlayerPlugin {
    fn build(&self, app:&mut App) {
        app
            .add_startup_system(spawn_player)
            .add_system(gamepad_connections.label("gamepads"))
            .add_system(keyboard_input.label("input"))
            .add_system(gamepad_input.label("input").after("gamepads"))
            .add_system(player_movement.label("player-movement").after("input").before("movement"))
            .add_system(camera_follow.after("movement"));
    }
}

fn camera_follow(player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (Without<Player>, With<Camera>)>) {
        let player_transform = player_query.single();
        let mut camera_transform = camera_query.single_mut();
        camera_transform.translation.x = player_transform.translation.x;
        camera_transform.translation.y = player_transform.translation.y;
    }

fn keyboard_input(
    mut player_query: Query<&mut Player>,
    keyboard: Res<Input<KeyCode>>
){
    let mut player = player_query.single_mut();
    player.up = keyboard.pressed(KeyCode::W);
    player.down = keyboard.pressed(KeyCode::S);
    player.left = keyboard.pressed(KeyCode::A);
    player.right = keyboard.pressed(KeyCode::D);
}

fn gamepad_input(
    mut player_query: Query<&mut Player>,
    axes: Res<Axis<GamepadAxis>>,
    // buttons: Res<Input<GamepadButton>>,
    my_gamepad: Option<Res<MyGamepad>>
) {
    let gamepad = if let Some(gp) = my_gamepad {
        // a gamepad is connected, we have the id
        gp.0
    } else {
        // no gamepad is connected
        return;
    };

    // The joysticks are represented using a separate axis for X and Y

    let axis_lx = GamepadAxis(gamepad, GamepadAxisType::LeftStickX);
    let axis_ly = GamepadAxis(gamepad, GamepadAxisType::LeftStickY);

    if let (Some(x), Some(y)) = (axes.get(axis_lx), axes.get(axis_ly)) {
        // combine X and Y into one vector
        let mut player = player_query.single_mut();
        player.stick_pos.x = x;
        player.stick_pos.y = y;
        player.up = y > 0.5;
        player.down = y < -0.5;
        player.left = x < -0.5;
        player.right = x > 0.5;
    }

    // // In a real game, the buttons would be configurable, but here we hardcode them
    // let jump_button = GamepadButton(gamepad, GamepadButtonType::South);
    // let heal_button = GamepadButton(gamepad, GamepadButtonType::East);

    // if buttons.just_pressed(jump_button) {
    //     // button just pressed: make the player jump
    // }

    // if buttons.pressed(heal_button) {
    //     // button being held down: heal the player
    // }
}

fn player_movement(
    mut player_query: Query<(&Player, &mut Accelleration)>
) {
    let (player, mut accel) = player_query.single_mut();
    let mut player_accel = accel.get_force("PLAYER_ACCEL");
    if player.up {
        player_accel.y = player.speed * TILE_SIZE;
    } else if player.down {
        player_accel.y = -player.speed * TILE_SIZE;
    } else {
        player_accel.y = 0.;
    }

    if player.left {
        player_accel.x = -player.speed * TILE_SIZE;
    } else if player.right {
        player_accel.x = player.speed * TILE_SIZE;
    } else {
        player_accel.x = 0.;
    }
    accel.set_force("PLAYER_ACCEL", player_accel);
}

/// Simple resource to store the ID of the connected gamepad.
/// We need to know which gamepad to use for player input.
struct MyGamepad(Gamepad);

fn gamepad_connections(
    mut commands: Commands,
    my_gamepad: Option<Res<MyGamepad>>,
    mut gamepad_evr: EventReader<GamepadEvent>,
) {
    for GamepadEvent(id, kind) in gamepad_evr.iter() {
        match kind {
            GamepadEventType::Connected => {
                println!("New gamepad connected with ID: {:?}", id);

                // if we don't have any gamepad yet, use this one
                if my_gamepad.is_none() {
                    commands.insert_resource(MyGamepad(*id));
                }
            }
            GamepadEventType::Disconnected => {
                println!("Lost gamepad connection with ID: {:?}", id);

                // if it's the one we previously associated with the player,
                // disassociate it:
                if let Some(MyGamepad(old_id)) = my_gamepad.as_deref() {
                    if old_id == id {
                        commands.remove_resource::<MyGamepad>();
                    }
                }
            }
            // other events are irrelevant
            _ => {}
        }
    }
}

fn spawn_player(mut commands: Commands, atlas: Res<CharacterAtlas>) {
    let player = spawn_sprite(
        &mut commands, 
        &atlas as &CharacterAtlas, 
        1, 
        Vec3::new(32.0,-320.0,900.0));
    
    commands.entity(player)
        .insert(Name::new("Player"))
        .insert(Velocity::new())
        .insert(Accelleration::new())
        .insert(Collidable{radius: CHARACTER_TILE_SIZE / 2.})
        .insert(Player{speed: 100.0, ..Default::default()}).id();
}