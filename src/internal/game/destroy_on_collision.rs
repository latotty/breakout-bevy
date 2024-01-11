use bevy::{prelude::*, utils::HashSet};

use crate::internal::physics::CollisionEvent;

#[derive(Component)]
pub enum DestroyOnCollision {
    This,
    Other,
}

pub fn handle_destroy_collision_events(
    mut commands: Commands,
    mut ev_collision: EventReader<CollisionEvent>,
    collision_query: Query<(Entity, &DestroyOnCollision)>,
) {
    let mut removed = HashSet::<Entity>::new();
    for collision in ev_collision.read() {
        for entity in collision.collidees {
            if removed.contains(&entity) {
                continue;
            }
            let other = collision.collidees.iter().find(|&&e| e != entity).unwrap();
            if let Ok((_, destroy)) = collision_query.get(entity) {
                match destroy {
                    DestroyOnCollision::This => {
                        commands.entity(entity).despawn();
                        removed.insert(entity);
                    },
                    DestroyOnCollision::Other => {
                        commands.entity(*other).despawn();
                        removed.insert(*other);
                    },
                }
            }
        }
    }
}
