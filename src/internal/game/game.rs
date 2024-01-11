use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::internal::{
    physics::{
        CollisionBody, CollisionGroup, CollisionMask, CollisionProperties, PhysicsLabel, Velocity,
    },
    states::AppState,
};

use super::{
    brick::BrickBlockFactory,
    destroy_on_collision::handle_destroy_collision_events,
    paddle::{move_paddle, PaddleFactory},
    score::{handle_score_on_collision_events, update_scoreboard, Scoreboard, ScoreboardFactory},
    wall::{EdgeWallFactory, BOTTOM_WALL, LEFT_WALL, RIGHT_WALL, TOP_WALL},
};

// We set the z-value of the ball to 1 so it renders on top in the case of overlapping sprites.
const BALL_SIZE: Vec3 = Vec3::new(20.0, 20.0, 0.0);
const BALL_STARTING_POSITION: Vec3 = Vec3::new(0.0, BOTTOM_WALL + 60. + 20. + 20., 1.0);
const BALL_STARTING_SPEED: f32 = 200.0;
const INITIAL_BALL_DIRECTION: Vec2 = Vec2::new(0.5, 0.5);

const BALL_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);

pub struct GamePlugin;

#[derive(Component)]
pub struct GameState;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), game_setup)
            .add_systems(OnExit(AppState::Game), game_teardown)
            .add_systems(
                FixedUpdate,
                (
                    move_paddle.before(PhysicsLabel),
                    (
                        handle_destroy_collision_events,
                        handle_score_on_collision_events,
                    )
                        .after(PhysicsLabel),
                )
                    .run_if(in_state(AppState::Game)),
            )
            .add_systems(
                Update,
                (update_scoreboard, esc_to_menu).run_if(in_state(AppState::Game)),
            );
    }
}

fn game_teardown(mut commands: Commands, query: Query<Entity, With<GameState>>) {
    commands.remove_resource::<Scoreboard>();
    for entity in query.iter() {
        // despawn the entity and its children
        commands.entity(entity).despawn_recursive();
    }
}

fn game_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(Scoreboard { score: 0 });

    commands
        .spawn((SpatialBundle::default(), GameState))
        .with_children(|parent| {
            PaddleFactory.spawn(parent);

            // Ball
            parent.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::default().into()).into(),
                    material: materials.add(ColorMaterial::from(BALL_COLOR)),
                    transform: Transform::from_translation(BALL_STARTING_POSITION)
                        .with_scale(BALL_SIZE),
                    ..default()
                },
                Velocity(INITIAL_BALL_DIRECTION.normalize() * BALL_STARTING_SPEED),
                CollisionProperties {
                    body: CollisionBody::Circle,
                    group: CollisionMask::new(&[CollisionGroup::Ball]),
                    mask: CollisionMask::new(&[
                        CollisionGroup::Block,
                        CollisionGroup::Wall,
                        CollisionGroup::Paddle,
                    ]),
                    ..default()
                },
            ));
            parent.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::default().into()).into(),
                    material: materials.add(ColorMaterial::from(BALL_COLOR)),
                    transform: Transform::from_translation(BALL_STARTING_POSITION)
                        .with_scale(BALL_SIZE),
                    ..default()
                },
                Velocity(INITIAL_BALL_DIRECTION.normalize() * BALL_STARTING_SPEED),
                CollisionProperties {
                    body: CollisionBody::Circle,
                    group: CollisionMask::new(&[CollisionGroup::Ball]),
                    mask: CollisionMask::new(&[
                        CollisionGroup::Block,
                        CollisionGroup::Wall,
                        CollisionGroup::Paddle,
                    ]),
                    ..default()
                },
            ));

            ScoreboardFactory.spawn(parent);

            EdgeWallFactory.spawn(parent);

            BrickBlockFactory {
                target_rect: Rect::from_corners(
                    Vec2::new(LEFT_WALL, BOTTOM_WALL + 120.),
                    Vec2::new(RIGHT_WALL, TOP_WALL),
                ),
                block_side_margin: 40.,
                block_top_margin: 40.,
                block_bottom_margin: 120.,
                brick_size: Vec2::new(60., 30.),
                brick_margin: 5.,
                brick_color: Color::rgb(0.5, 0.5, 1.0),
            }
            .spawn(parent);
        });
}

pub fn esc_to_menu(mut next_state: ResMut<NextState<AppState>>, input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::Escape) {
        next_state.set(AppState::MainMenu);
    }
}
