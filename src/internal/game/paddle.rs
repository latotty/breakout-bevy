use bevy::prelude::*;

use crate::internal::physics::{
    CollisionBody, CollisionGroup, CollisionMask, CollisionProperties, CurvedRectBounce,
};

use super::wall::{BOTTOM_WALL, LEFT_WALL, RIGHT_WALL, WALL_THICKNESS};

const PADDLE_PADDING: f32 = 10.0;
const PADDLE_SPEED: f32 = 500.0;
const PADDLE_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
const GAP_BETWEEN_PADDLE_AND_FLOOR: f32 = 60.0;

#[derive(Component)]
pub struct Paddle;

pub struct PaddleFactory;

impl PaddleFactory {
    pub fn spawn(&self, parent: &mut ChildBuilder) {
        parent.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(0.0, BOTTOM_WALL + GAP_BETWEEN_PADDLE_AND_FLOOR, 0.0),
                    scale: Vec3::new(120.0, 20.0, 0.0),
                    ..default()
                },
                sprite: Sprite {
                    color: PADDLE_COLOR,
                    ..default()
                },
                ..default()
            },
            Paddle,
            CollisionProperties {
                body: CollisionBody::Rect,
                group: CollisionMask::new(&[CollisionGroup::Paddle]),
                mask: CollisionMask::new(&[CollisionGroup::Ball, CollisionGroup::Powerup]),
                ..default()
            },
            CurvedRectBounce {
                curvature: core::f32::consts::FRAC_PI_3,
            },
        ));
    }
}

pub fn move_paddle(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Paddle>>,
    time: Res<Time>,
) {
    let mut paddle_transform = query.single_mut();
    let paddle_size = paddle_transform.scale.truncate();
    let direction = if keyboard_input.pressed(KeyCode::Left) {
        -1.0
    } else if keyboard_input.pressed(KeyCode::Right) {
        1.0
    } else {
        return;
    };

    // Calculate the new horizontal paddle position based on player input
    let new_paddle_position =
        paddle_transform.translation.x + direction * PADDLE_SPEED * time.delta_seconds();

    // Update the paddle position,
    // making sure it doesn't cause the paddle to leave the arena
    let left_bound = LEFT_WALL + WALL_THICKNESS / 2.0 + paddle_size.x / 2.0 + PADDLE_PADDING;
    let right_bound = RIGHT_WALL - WALL_THICKNESS / 2.0 - paddle_size.x / 2.0 - PADDLE_PADDING;

    paddle_transform.translation.x = new_paddle_position.clamp(left_bound, right_bound);
}
