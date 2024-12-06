use bevy::prelude::*;

mod board;
mod camera;
mod globals;
mod graphics;
mod input;
mod pieces;
mod player;
mod states;
mod vectors;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Tomb of the Mask".to_string(),
                        resolution: (globals::WINDOW_WIDTH, globals::WINDOW_HEIGHT).into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .init_state::<states::MainState>()
        .add_plugins((
            board::BoardPlugin,
            graphics::GraphicsPlugin,
            input::InputPlugin,
            player::PlayerPlugin,
        ))
        .add_systems(Startup, camera::setup)
        .run();
}
