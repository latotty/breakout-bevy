use bevy::prelude::*;

use crate::internal::physics::{CollisionBody, CollisionGroup, CollisionMask, CollisionProperties};

use super::destroy_on_collision::DestroyOnCollision;

pub const WALL_THICKNESS: f32 = 10.0;

pub const LEFT_WALL: f32 = -450.;
pub const RIGHT_WALL: f32 = 450.;
pub const BOTTOM_WALL: f32 = -300.;
pub const TOP_WALL: f32 = 300.;

const WALL_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);

#[derive(Bundle)]
pub struct WallBundle {
    sprite_bundle: SpriteBundle,
    collision_properties: CollisionProperties,
}

pub enum EdgeWallLocation {
    Left,
    Right,
    Bottom,
    Top,
}

impl EdgeWallLocation {
    fn position(&self) -> Vec2 {
        match self {
            EdgeWallLocation::Left => Vec2::new(LEFT_WALL, 0.),
            EdgeWallLocation::Right => Vec2::new(RIGHT_WALL, 0.),
            EdgeWallLocation::Bottom => Vec2::new(0., BOTTOM_WALL),
            EdgeWallLocation::Top => Vec2::new(0., TOP_WALL),
        }
    }

    fn size(&self) -> Vec2 {
        let arena_height = TOP_WALL - BOTTOM_WALL;
        let arena_width = RIGHT_WALL - LEFT_WALL;

        assert!(arena_height > 0.0);
        assert!(arena_width > 0.0);

        match self {
            EdgeWallLocation::Left | EdgeWallLocation::Right => {
                Vec2::new(WALL_THICKNESS, arena_height + WALL_THICKNESS)
            }
            EdgeWallLocation::Bottom | EdgeWallLocation::Top => {
                Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS)
            }
        }
    }
}

impl From<EdgeWallLocation> for WallBundle {
    fn from(location: EdgeWallLocation) -> WallBundle {
        WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: location.position().extend(0.0),
                    scale: location.size().extend(1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    ..default()
                },
                ..default()
            },
            collision_properties: CollisionProperties {
                body: CollisionBody::Rect,
                group: CollisionMask::new(&[CollisionGroup::Wall]),
                mask: CollisionMask::new(&[CollisionGroup::Ball, CollisionGroup::Powerup]),
                ..default()
            },
        }
    }
}

pub struct EdgeWallFactory;

impl EdgeWallFactory {
    pub fn spawn(&self, parent: &mut ChildBuilder) {
        parent.spawn(WallBundle::from(EdgeWallLocation::Left));
        parent.spawn(WallBundle::from(EdgeWallLocation::Right));
        parent.spawn((
            WallBundle::from(EdgeWallLocation::Bottom),
            DestroyOnCollision::Other,
        ));
        parent.spawn(WallBundle::from(EdgeWallLocation::Top));
    }
}
