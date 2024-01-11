use bevy::prelude::*;

use super::{
    collision::{circle_circle_collision, rect_circle_collision, rect_rect_collision},
    velocity::Velocity,
    CollisionBody, CollisionProperties,
};

#[derive(Event, Debug)]
pub struct CollisionEvent {
    pub collidees: [Entity; 2],
    pub collision_result: CollisionResult,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CollisionResult {
    pub collision_normal: Vec2,
    pub corrigation_vector: Vec2,
}

pub fn check_for_collisions(
    mut ev_collision: EventWriter<CollisionEvent>,
    collision_query: Query<(Entity, &Transform, &CollisionProperties)>,
) {
    for [(a_entity, a_transform, a_collision_properties), (b_entity, b_transform, b_collision_properties)] in
        collision_query.iter_combinations()
    {
        if !a_collision_properties
            .mask
            .contains(&b_collision_properties.group)
            || !b_collision_properties
                .mask
                .contains(&a_collision_properties.group)
        {
            continue;
        }

        let (
            (a_entity, a_transform, a_collision_properties),
            (b_entity, b_transform, b_collision_properties),
        ) = match (a_collision_properties.body, b_collision_properties.body) {
            (CollisionBody::Circle, CollisionBody::Rect) => (
                (b_entity, b_transform, b_collision_properties),
                (a_entity, a_transform, a_collision_properties),
            ),

            (CollisionBody::Rect, CollisionBody::Rect)
            | (CollisionBody::Rect, CollisionBody::Circle)
            | (CollisionBody::Circle, CollisionBody::Circle) => (
                (a_entity, a_transform, a_collision_properties),
                (b_entity, b_transform, b_collision_properties),
            ),
        };

        let collision_result: Option<CollisionResult> =
            match (a_collision_properties.body, b_collision_properties.body) {
                (CollisionBody::Rect, CollisionBody::Rect) => rect_rect_collision(
                    a_transform.translation.truncate(),
                    a_transform.scale.truncate(),
                    b_transform.translation.truncate(),
                    b_transform.scale.truncate(),
                ),
                (CollisionBody::Circle, CollisionBody::Circle) => circle_circle_collision(
                    a_transform.translation.truncate(),
                    a_transform.scale.truncate(),
                    b_transform.translation.truncate(),
                    b_transform.scale.truncate(),
                ),
                (CollisionBody::Rect, CollisionBody::Circle) => rect_circle_collision(
                    a_transform.translation.truncate(),
                    a_transform.scale.truncate(),
                    b_transform.translation.truncate(),
                    b_transform.scale.truncate(),
                ),
                _ => panic!("Invalid collision body combination"),
            };

        if let Some(collision_result) = collision_result {
            ev_collision.send(CollisionEvent {
                collidees: [a_entity, b_entity],
                collision_result,
            });
        }
    }
}

pub fn handle_correction(
    mut ev_collision: EventReader<CollisionEvent>,
    mut collision_query: Query<(&mut Transform, Option<&Velocity>)>,
) {
    for collision in ev_collision.read() {
        let entries = collision_query.get_many_mut(collision.collidees);
        if let Ok(mut entries) = entries {
            let corrigation_vectors = match (entries[0].1.is_some(), entries[1].1.is_some()) {
                (true, true) => [
                    collision.collision_result.corrigation_vector / 2.,
                    collision.collision_result.corrigation_vector / -2.,
                ],
                (true, false) => [collision.collision_result.corrigation_vector, Vec2::ZERO],
                (false, true) => [Vec2::ZERO, collision.collision_result.corrigation_vector],
                (false, false) => [Vec2::ZERO, Vec2::ZERO],
            };
            entries[0].0.translation += corrigation_vectors[0].extend(0.0);
            entries[1].0.translation += corrigation_vectors[1].extend(0.0);
        }
    }
}

pub fn handle_collision_bounce(
    mut ev_collision: EventReader<CollisionEvent>,
    mut collidee_query: Query<(Option<&mut Velocity>, &CollisionProperties)>,
) {
    for collision in ev_collision.read() {
        let collidees = collidee_query.get_many_mut(collision.collidees);

        match collidees {
            Ok([(Some(mut velocity), moving_collision_props), (None, stat_collision_props)])
            | Ok([(None, stat_collision_props), (Some(mut velocity), moving_collision_props)]) => {
                let new_velocity = velocity.0 * -1.0;

                let velocity_speed = new_velocity.length();
                let velocity_speed = velocity_speed
                    * moving_collision_props.bounciness
                    * stat_collision_props.bounciness;

                let velocity_angle = collision
                    .collision_result
                    .collision_normal
                    .angle_between(new_velocity)
                    * -1.;
                const MAX_ANGLE: f32 = std::f32::consts::FRAC_PI_2 / 9. * 8.;
                let velocity_angle = velocity_angle.clamp(-MAX_ANGLE, MAX_ANGLE);
                let new_velocity = collision
                    .collision_result
                    .collision_normal
                    .rotate(Vec2::from_angle(velocity_angle))
                    * velocity_speed;

                velocity.x = new_velocity.x;
                velocity.y = new_velocity.y;
            }
            // TODO - moving moving collision
            // Ok([Some(mut a_velocity), Some(mut b_velocity)]) => {
            //     a_velocity.x *= -1.;
            //     a_velocity.y *= -1.;
            //     b_velocity.x *= -1.;
            //     b_velocity.y *= -1.;
            // }
            _ => panic!("Invalid collision"),
        }
    }
}
