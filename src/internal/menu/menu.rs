use bevy::prelude::*;

use crate::internal::states::AppState;

pub struct MenuPlugin;

#[derive(Component)]
pub struct MenuState;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), menu_setup)
            .add_systems(OnExit(AppState::MainMenu), menu_teardown)
            .add_systems(
                Update,
                (bevy::window::close_on_esc, space_to_start).run_if(in_state(AppState::MainMenu)),
            );
    }
}

fn menu_teardown(mut commands: Commands, query: Query<Entity, With<MenuState>>) {
    for entity in query.iter() {
        // despawn the entity and its children
        commands.entity(entity).despawn_recursive();
    }
}

fn menu_setup(mut commands: Commands) {
    commands.spawn((NodeBundle { 
        style: Style {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            ..Default::default()
        },
        ..Default::default() 
    }, MenuState)).with_children(|parent| {
        parent.spawn(
            TextBundle::from_sections([TextSection::new(
                "Press Space to Start",
                TextStyle {
                    font_size: 40.,
                    color: Color::RED,
                    ..default()
                },
            )]),
        );
    });
}

pub fn space_to_start(mut next_state: ResMut<NextState<AppState>>, input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::Space) {
        next_state.set(AppState::Game);
    }
}
