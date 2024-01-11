use bevy::prelude::*;

use super::super::collider::CollisionResult;

pub fn rect_rect_collision(a_pos: Vec2, a_size: Vec2, b_pos: Vec2, b_size: Vec2) -> Option<CollisionResult> {
    let a_min = a_pos - a_size / 2.0;
    let a_max = a_pos + a_size / 2.0;

    let b_min = b_pos - b_size / 2.0;
    let b_max = b_pos + b_size / 2.0;

    // check to see if the two rectangles are intersecting
    if a_min.x < b_max.x && a_max.x > b_min.x && a_min.y < b_max.y && a_max.y > b_min.y {
        // check to see if we hit on the left or right side
        let (x_collision, x_depth) = if a_min.x < b_min.x && a_max.x > b_min.x && a_max.x < b_max.x
        {
            (Vec2::new(1.0, 0.), (b_min.x - a_max.x).abs())
        } else if a_min.x > b_min.x && a_min.x < b_max.x && a_max.x > b_max.x {
            (Vec2::new(-1.0, 0.), (a_min.x - b_max.x).abs())
        } else if a_pos.x < b_pos.x {
            (Vec2::new(1.0, 0.), (b_min.x - a_max.x).abs())
        } else {
            (Vec2::new(-1.0, 0.), (a_min.x - b_max.x).abs())
        };

        // check to see if we hit on the top or bottom side
        let (y_collision, y_depth) = if a_min.y < b_min.y && a_max.y > b_min.y && a_max.y < b_max.y
        {
            (Vec2::new(0.0, 1.0), (b_min.y - a_max.y).abs())
        } else if a_min.y > b_min.y && a_min.y < b_max.y && a_max.y > b_max.y {
            (Vec2::new(0.0, -1.0), (a_min.y - b_max.y).abs())
        } else if a_pos.y < b_pos.y {
            (Vec2::new(0.0, 1.0), (b_min.y - a_max.y).abs())
        } else {
            (Vec2::new(0.0, -1.0), (a_min.y - b_max.y).abs())
        };

        // if we had an "x" and a "y" collision, pick the "primary" side using penetration depth
        if y_depth < x_depth {
            Some(CollisionResult {
                collision_normal: y_collision,
                corrigation_vector: y_collision * y_depth,
            })
        } else {
            Some(CollisionResult {
                collision_normal: x_collision,
                corrigation_vector: x_collision * x_depth,
            })
        }
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
        Some(CollisionResult{
            collision_normal: Vec2::new(1.0, 0.0),
            corrigation_vector: Vec2::new(1.0, 0.0),
        })
    )]
    #[case(
        Vec2::new(9.0, 0.0),
        Vec2::new(10.0, 10.0),
        Vec2::new(0.0, 0.0),
        Vec2::new(10.0, 10.0),
        Some(CollisionResult{
            collision_normal: Vec2::new(-1.0, 0.0),
            corrigation_vector: Vec2::new(-1.0, 0.0),
        })
    )]
    #[case(
        Vec2::new(0.0, 0.0),
        Vec2::new(10.0, 10.0),
        Vec2::new(8.0, 0.0),
        Vec2::new(10.0, 10.0),
        Some(CollisionResult{
            collision_normal: Vec2::new(1.0, 0.0),
            corrigation_vector: Vec2::new(2.0, 0.0),
        })
    )]
    fn test_rect_rect_collision(
        #[case] a_pos: Vec2,
        #[case] a_size: Vec2,
        #[case] b_pos: Vec2,
        #[case] b_size: Vec2,
        #[case] expected: Option<CollisionResult>,
    ) {
        let collision_vector = rect_rect_collision(a_pos, a_size, b_pos, b_size);

        assert_eq!(collision_vector, expected);
    }
}
