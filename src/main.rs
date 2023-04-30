use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowResolution},
};
mod entities;
mod plugins;
mod utils;
use entities::player::Ship;
use plugins::tile_map::*;
use utils::structs::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(1280.0, 720.0),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_startup_system(setup)
        .add_system(Ship::move_ship)
        .run();
}

fn setup(
    mut commands: Commands,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let mut window = window_query.get_single_mut().unwrap();

    window.title = "Rust Game".to_string();

    let window_size = WindowSize {
        height: window.height(),
        width: window.width(),
    };

    commands.insert_resource(WindowSize {
        height: window.height(),
        width: window.width(),
    });

    commands
        .spawn(Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        })
        .insert(MainCamera {});

    create_water_map(&mut commands, &asset_server);

    Ship::spawn_player(commands, asset_server, window_size);

    println!("Setup was successful");
}
