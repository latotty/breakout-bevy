use bevy::prelude::*;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CollisionGroup {
    Paddle = 1,
    Ball = 2,
    Block = 4,
    Wall = 8,
    Powerup = 16,
}
pub const COLLISION_GROUPS: [CollisionGroup; 5] = [
    CollisionGroup::Paddle,
    CollisionGroup::Ball,
    CollisionGroup::Block,
    CollisionGroup::Wall,
    CollisionGroup::Powerup,
];

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct CollisionMask(u16);

impl CollisionMask {
    pub fn new(groups: &[CollisionGroup]) -> Self {
        let mut mask = Self(0);
        for group in groups {
            mask = mask.with(*group);
        }
        mask
    }

    pub fn with(mut self, group: CollisionGroup) -> Self {
        self.0 |= group as u16;
        self
    }

    pub fn without(mut self, group: CollisionGroup) -> Self {
        self.0 &= !(group as u16);
        self
    }

    pub fn contains(&self, other: &CollisionMask) -> bool {
        self.0 & other.0 != 0
    }

    pub fn contains_group(&self, group: CollisionGroup) -> bool {
        self.0 & (group as u16) != 0
    }
}

impl fmt::Display for CollisionMask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        let mut groups = Vec::new();
        for group in COLLISION_GROUPS.iter() {
            if self.contains_group(*group) {
                groups.push(format!("{:?}", group));
            }
        }
        write!(f, "CollisionMask({})", groups.join(" | "))
    }
}

impl fmt::Debug for CollisionMask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum CollisionBody {
    #[default]
    Rect,
    Circle,
}

#[derive(Component)]
pub struct CollisionProperties {
    pub body: CollisionBody,
    pub group: CollisionMask,
    pub mask: CollisionMask,
    pub bounciness: f32,
}

impl Default for CollisionProperties {
    fn default() -> Self {
        Self {
            body: CollisionBody::default(),
            group: CollisionMask::default(),
            mask: CollisionMask::default(),
            bounciness: 1.0,
        }
    }
}
