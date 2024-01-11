use bevy::prelude::*;

use super::{CollisionEvent, Velocity};

#[derive(Component)]
pub struct CurvedRectBounce {
    pub curvature: f32,
}

pub fn handle_curved_bounce(
    mut ev_collision: EventReader<CollisionEvent>,
    curved_query: Query<(&CurvedRectBounce, &Transform)>,
    mut other_query: Query<(&mut Velocity, &Transform)>,
) {
    for collision in ev_collision.read() {
        let curved_rect = if let Ok(curved_rect) = curved_query.get(collision.collidees[0]) {
            curved_rect
        } else {
            continue;
        };
        let (mut velocity, other_transform) =
            if let Ok(other) = other_query.get_mut(collision.collidees[1]) {
                other
            } else {
                continue;
            };

        let rect_center = curved_rect.1.translation;
        let rect_size = curved_rect.1.scale;
        let rect_curvature = curved_rect.0.curvature;
        let circle_center = other_transform.translation;

        let collision_normal = collision.collision_result.collision_normal;

        let collision_ratio = if collision_normal.x == 1.0 && collision_normal.y == 0.0 {
            (rect_center.y - circle_center.y) / (rect_size.y / 2.0)
        } else if collision_normal.x == -1.0 && collision_normal.y == 0.0 {
            (circle_center.y - rect_center.y) / (rect_size.y / 2.0)
        } else if collision_normal.x == 0.0 && collision_normal.y == 1.0 {
            (rect_center.x - circle_center.x) / (rect_size.x / 2.0)
        } else if collision_normal.x == 0.0 && collision_normal.y == -1.0 {
            (circle_center.x - rect_center.x) / (rect_size.x / 2.0)
        } else {
            panic!("Invalid collision normal")
        };
        let collision_ratio = collision_ratio.clamp(-1.0, 1.0);
        let collision_angle = collision_ratio * rect_curvature;

        let velocity_speed = velocity.length();

        let velocity_angle = collision
            .collision_result
            .collision_normal
            .angle_between(velocity.0)
            + collision_angle;
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
}
