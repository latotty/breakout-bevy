use bevy::prelude::*;

use crate::internal::{
    game::destroy_on_collision::DestroyOnCollision,
    physics::{CollisionBody, CollisionGroup, CollisionMask, CollisionProperties},
};

use super::score::ScoreOnCollision;

/**
 * Brick Factory
 */
pub struct BrickBlockFactory {
    pub target_rect: Rect,
    pub brick_size: Vec2,
    pub brick_margin: f32,
    pub block_side_margin: f32,
    pub block_top_margin: f32,
    pub block_bottom_margin: f32,
    pub brick_color: Color,
}
impl BrickBlockFactory {
    pub fn spawn(&self, parent: &mut ChildBuilder) {
        let right_edge = self.target_rect.max.x;
        let left_edge = self.target_rect.min.x;
        let top_edge = self.target_rect.max.y;
        let bottom_edge = self.target_rect.min.y;

        let total_width_of_bricks = (right_edge - left_edge) - 2. * self.block_side_margin;
        let top_edge_of_bricks = top_edge - self.block_top_margin;
        let bottom_edge_of_bricks = bottom_edge + self.block_bottom_margin;
        let total_height_of_bricks = top_edge - bottom_edge_of_bricks - self.block_top_margin;

        assert!(total_width_of_bricks > 0.0);
        assert!(total_height_of_bricks > 0.0);

        // Given the space available, compute how many rows and columns of bricks we can fit
        let n_columns =
            (total_width_of_bricks / (self.brick_size.x + self.brick_margin)).floor() as usize;
        let n_rows =
            (total_height_of_bricks / (self.brick_size.y + self.brick_margin)).floor() as usize;
        let n_vertical_gaps = n_columns - 1;

        // Because we need to round the number of columns,
        // the space on the top and sides of the bricks only captures a lower bound, not an exact value
        let center_of_bricks = (left_edge + right_edge) / 2.0;
        let left_edge_of_bricks = center_of_bricks
            // Space taken up by the bricks
            - (n_columns as f32 / 2.0 * self.brick_size.x)
            // Space taken up by the gaps
            - n_vertical_gaps as f32 / 2.0 * self.brick_margin;

        // In Bevy, the `translation` of an entity describes the center point,
        // not its bottom-left corner
        let offset_x = left_edge_of_bricks + self.brick_size.x / 2.;
        let offset_y = top_edge_of_bricks - self.brick_size.y / 2.;

        for row in 0..n_rows {
            for column in 0..n_columns {
                let brick_position = Vec2::new(
                    offset_x + column as f32 * (self.brick_size.x + self.brick_margin),
                    offset_y - row as f32 * (self.brick_size.y + self.brick_margin),
                );

                // brick
                parent.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: self.brick_color,
                            ..default()
                        },
                        transform: Transform {
                            translation: brick_position.extend(0.0),
                            scale: Vec3::new(self.brick_size.x, self.brick_size.y, 1.0),
                            ..default()
                        },
                        ..default()
                    },
                    CollisionProperties {
                        body: CollisionBody::Rect,
                        group: CollisionMask::new(&[CollisionGroup::Block]),
                        mask: CollisionMask::new(&[CollisionGroup::Ball]),
                        ..default()
                    },
                    DestroyOnCollision::This,
                    ScoreOnCollision(1),
                ));
            }
        }
    }
}
