use bevy::{math::vec2, prelude::*};
use physics_manager::PhysicsEntity;

use crate::physics;
use collisions::{Collider, ColliderType};
use physics::*;


use super::entities::*;


#[derive(Resource, Default)]
pub struct KeyboardInputs{
    pub left: bool,
    pub right: bool,
    pub jump: bool,
}

pub fn setup_player(
    mut commands: Commands, 
    asset_server: Res<AssetServer>
)
{ 
    commands.spawn((
        SpriteBundle{
            texture: asset_server.load("test_sprite.png"),
            transform: Transform::from_scale(Vec3::splat(1.)).with_translation(Vec3::new(0., 150., 0.,)),
            ..default()
        },
        KeyboardMovable,
        EntityValues{
            speed: 5.0,
            jump_height: 5.0,
            is_grounded: false,
        },
        PhysicsEntity{
            weight: 0.75,
            velocity: vec2(0., 0.)
        },
        Collider{
            size_x: 0.75,
            size_y: 1.5,
            collider_type: ColliderType::Cube,
        }
    ));
}  
pub fn move_player(
    mut query: Query<(&mut Transform, &mut EntityValues, &mut PhysicsEntity), With<KeyboardMovable>>,
    keyboard_inputs: ResMut<KeyboardInputs>,
) {
    for (mut transform, mut entity_values, mut physics) in query.iter_mut() {
        if keyboard_inputs.left {
            transform.translation.x -= entity_values.speed;
        }
        if keyboard_inputs.right {
            transform.translation.x += entity_values.speed;
        }

        if keyboard_inputs.jump && entity_values.is_grounded{
            //jump
            physics.velocity.y += entity_values.jump_height;
            entity_values.is_grounded = false;
            println!("jumping");
        }
    }
} 

pub fn keyboard_input(
    mut keyboard_inputs: ResMut<KeyboardInputs>,
    keys: Res<ButtonInput<KeyCode>>,
){

    keyboard_inputs.left = keys.pressed(KeyCode::KeyA);

    keyboard_inputs.right = keys.pressed(KeyCode::KeyD);

    keyboard_inputs.jump = keys.just_pressed(KeyCode::Space);
}