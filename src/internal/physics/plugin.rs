use bevy::prelude::*;

use super::{
    apply_velocity,
    collider::{check_for_collisions, handle_collision_bounce, handle_correction, CollisionEvent},
    handle_curved_bounce,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct PhysicsLabel;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollisionEvent>().add_systems(
            FixedUpdate,
            (
                apply_velocity,
                check_for_collisions,
                handle_correction,
                handle_collision_bounce,
                handle_curved_bounce,
            )
                .chain()
                .in_set(PhysicsLabel),
        );
    }
}
