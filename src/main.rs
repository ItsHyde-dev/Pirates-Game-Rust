use bevy::{prelude::*, window::PrimaryWindow};
mod entities;
mod plugins;
mod utils;
use entities::player::Ship;
use plugins::tile_map::*;
use utils::structs::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
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

    window.set_maximized(true);

    let window_size = WindowSize {
        height: window.height(),
        width: window.width(),
    };

    let water: Handle<Image> = asset_server.load("water.png");
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(
            (window.width() / 2.0) + 64.0,
            (window.height() / 2.0) + 64.0,
            0.0,
        ),
        ..default()
    });

    create_water_map(&mut commands, water.clone());

    Ship::spawn_player(commands, asset_server, window_size);

    println!("Setup was successful");
}
