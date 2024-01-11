use bevy::prelude::*;

use super::super::collider::CollisionResult;

pub fn circle_circle_collision(
    a_pos: Vec2,
    a_size: Vec2,
    b_pos: Vec2,
    b_size: Vec2,
) -> Option<CollisionResult> {
    let a_radius = a_size.x / 2.0;
    let b_radius = b_size.x / 2.0;

    let distance = a_pos.distance(b_pos);

    if distance < a_radius + b_radius {
        let collision_vector = (b_pos - a_pos).normalize();
        Some(CollisionResult {
            collision_normal: collision_vector,
            corrigation_vector: collision_vector * (a_radius + b_radius - distance),
        })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(
        Vec2::new(0.0, 0.0),
        Vec2::new(10.0, 10.0),
        Vec2::new(40.0, 0.0),
        Vec2::new(10.0, 10.0),
        None
    )]
    #[case(
        Vec2::new(0.0, 0.0),
        Vec2::new(10.0, 10.0),
        Vec2::new(0.0, 40.0),
        Vec2::new(10.0, 10.0),
        None
    )]
    #[case(
        Vec2::new(0.0, 0.0),
        Vec2::new(10.0, 10.0),
        Vec2::new(9.0, 0.0),
        Vec2::new(10.0, 10.0),
        Some(CollisionResult { 
            collision_normal: Vec2::new(1.0, 0.0),
            corrigation_vector: Vec2::new(1.0, 0.0),
        })
    )]
    #[case(
        Vec2::new(9.0, 0.0),
        Vec2::new(10.0, 10.0),
        Vec2::new(0.0, 0.0),
        Vec2::new(10.0, 10.0),
        Some(CollisionResult {
            collision_normal: Vec2::new(-1.0, 0.0),
            corrigation_vector: Vec2::new(-1.0, 0.0),
        }),
    )]
    #[case(
        Vec2::new(8.0, 0.0),
        Vec2::new(10.0, 10.0),
        Vec2::new(0.0, 0.0),
        Vec2::new(10.0, 10.0),
        Some(CollisionResult {
            collision_normal: Vec2::new(-1.0, 0.0),
            corrigation_vector: Vec2::new(-2.0, 0.0),
        }),
    )]
    #[case(
        Vec2::new(0.0, 0.0),
        Vec2::new(10.0, 10.0),
        Vec2::new(4.0, 4.0),
        Vec2::new(10.0, 10.0),
        Some(CollisionResult{
            collision_normal: Vec2::new(0.5, 0.5).normalize(),
            corrigation_vector: Vec2::new(0.5, 0.5).normalize() * (10.0 - Vec2::new(0.0, 0.0).distance(Vec2::new(4.0, 4.0))),
        } ),
    )]
    fn test_circle_circle_collision(
        #[case] a_pos: Vec2,
        #[case] a_size: Vec2,
        #[case] b_pos: Vec2,
        #[case] b_size: Vec2,
        #[case] expected: Option<CollisionResult>,
    ) {
        let collision_vector = circle_circle_collision(a_pos, a_size, b_pos, b_size);

        assert_eq!(collision_vector, expected);
    }
}
