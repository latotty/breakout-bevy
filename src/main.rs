use bevy::prelude::*;
use breakout_bevy::internal::{
    camera::CameraPlugin, game::game::GamePlugin, menu::menu::MenuPlugin, states::AppState, physics::PhysicsPlugin,
};

const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugin)
        .add_plugins(GamePlugin)
        .add_plugins(MenuPlugin)
        .add_plugins(CameraPlugin)
        .add_state::<AppState>()
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .run();
}
