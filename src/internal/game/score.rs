use bevy::{prelude::*, utils::HashSet};

use super::super::physics::CollisionEvent;

const SCOREBOARD_FONT_SIZE: f32 = 40.0;
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);

const TEXT_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
const SCORE_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);

#[derive(Resource)]
pub struct Scoreboard {
    pub score: usize,
}

pub fn update_scoreboard(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut();
    text.sections[1].value = scoreboard.score.to_string();
}

#[derive(Component)]
pub struct ScoreOnCollision(pub usize);

pub fn handle_score_on_collision_events(
    mut scoreboard: ResMut<Scoreboard>,
    mut ev_collision: EventReader<CollisionEvent>,
    collision_query: Query<(Entity, &ScoreOnCollision)>,
) {
    let mut scored = HashSet::<Entity>::new();
    for collision in ev_collision.read() {
        for entity in collision.collidees {
            if scored.contains(&entity) {
                continue;
            }
            if let Ok((_, score)) = collision_query.get(entity) {
                scoreboard.score += score.0;
                scored.insert(entity);
            }
        }
    }
}

pub struct ScoreboardFactory;

impl ScoreboardFactory {
    pub fn spawn(&self, parent: &mut ChildBuilder) {
        parent.spawn(
            TextBundle::from_sections([
                TextSection::new(
                    "Score: ",
                    TextStyle {
                        font_size: SCOREBOARD_FONT_SIZE,
                        color: TEXT_COLOR,
                        ..default()
                    },
                ),
                TextSection::from_style(TextStyle {
                    font_size: SCOREBOARD_FONT_SIZE,
                    color: SCORE_COLOR,
                    ..default()
                }),
            ])
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: SCOREBOARD_TEXT_PADDING,
                left: SCOREBOARD_TEXT_PADDING,
                ..default()
            }),
        );
    }
}
