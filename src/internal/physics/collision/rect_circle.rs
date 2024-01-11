use bevy::prelude::*;

use super::super::collider::CollisionResult;

pub fn rect_circle_collision(
    rect_center: Vec2,
    rect_size: Vec2,
    circle_center: Vec2,
    circle_size: Vec2,
) -> Option<CollisionResult> {
    let rect_half_size = rect_size / 2.0;

    let circle_radius = circle_size.x / 2.0;

    let circle_distance = (circle_center - rect_center).abs();

    if circle_distance.x > rect_half_size.x + circle_radius
        || circle_distance.y > rect_half_size.y + circle_radius
    {
        return None;
    }

    if circle_distance.x > rect_half_size.x && circle_distance.y > rect_half_size.y {
        let corner_distance_squared = (circle_distance - rect_half_size).length_squared();
        if corner_distance_squared > circle_radius * circle_radius {
            return None;
        }
    }

    let adjusted_circle_distance = circle_distance - rect_half_size;
    let collision_vector = if adjusted_circle_distance.x > adjusted_circle_distance.y {
        if circle_center.x > rect_center.x {
            Vec2::new(1.0, 0.0)
        } else {
            Vec2::new(-1.0, 0.0)
        }
    } else if circle_center.y > rect_center.y {
        Vec2::new(0.0, 1.0)
    } else {
        Vec2::new(0.0, -1.0)
    };

    Some(CollisionResult {
        collision_normal: collision_vector,
        corrigation_vector: collision_vector * -(adjusted_circle_distance - circle_radius),
    })
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
        Vec2::new(8.0, 0.0),
        Vec2::new(10.0, 10.0),
        Some(CollisionResult {
            collision_normal: Vec2::new(1.0, 0.0),
            corrigation_vector: Vec2::new(2.0, 0.0),
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
        })
    )]
    #[case(
        Vec2::new(0.0, 0.0),
        Vec2::new(10.0, 10.0),
        Vec2::new(8.0, 9.0),
        Vec2::new(10.0, 10.0),
        Some(CollisionResult {
            collision_normal: Vec2::new(0.0, 1.0),
            corrigation_vector: Vec2::new(0.0, 1.0),
        })
    )]
    #[case(
        Vec2::new(0.0, 0.0),
        Vec2::new(10.0, 10.0),
        Vec2::new(9.0, 9.0),
        Vec2::new(10.0, 10.0),
        None
    )]
    fn test_rect_circle_collision(
        #[case] a_pos: Vec2,
        #[case] a_size: Vec2,
        #[case] b_pos: Vec2,
        #[case] b_size: Vec2,
        #[case] expected: Option<CollisionResult>,
    ) {
        let collision_vector = rect_circle_collision(a_pos, a_size, b_pos, b_size);

        assert_eq!(collision_vector, expected);
    }
}
