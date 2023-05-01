use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use bevy::prelude::*;

use crate::utils::structs::{BlockingStructure, WaterTile, WindowSize};

pub fn create_water_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("Started Creating the background world");

    // use when we need to generate map using resource

    let file =
        File::open("assets/map 1._Tile Layer 2.csv").expect("There was an error reading the map");

    let map_1 = asset_server.load("map 1.png");

    for (y, line) in BufReader::new(file).lines().enumerate() {
        if let Ok(line) = line {
            for (x, c) in line.split(",").enumerate() {
                let c = c.trim();

                let x_t: f32 = ((x * 64) + 32) as f32;
                let y_t: f32 = ((y * 64) + 32) as f32;

                if c == "-1" {
                    commands
                        .spawn(SpriteBundle {
                            sprite: Sprite {
                                color: Color::rgba(0.0, 0.0, 5.0, 0.2),
                                custom_size: Some(Vec2::new(64.0, 64.0)),
                                ..default()
                            },
                            transform: Transform::from_xyz(x_t, y_t, 0.0),
                            global_transform: GlobalTransform::default(),
                            ..default()
                        })
                        .insert(WaterTile {});
                } else {
                    commands
                        .spawn(SpriteBundle {
                            sprite: Sprite {
                                color: Color::rgba(5.0, 0.0, 0.0, 0.2),
                                custom_size: Some(Vec2::new(64.0, 64.0)),
                                ..default()
                            },
                            transform: Transform::from_xyz(x_t, y_t, 0.0),
                            global_transform: GlobalTransform::default(),
                            ..default()
                        })
                        .insert(BlockingStructure {});
                }
            }
        }
    }

    let map_dimensions = WindowSize {
        height: 2560.0,
        width: 3840.0,
    };

    commands.spawn(SpriteBundle {
        texture: map_1,
        transform: Transform::from_xyz(
            map_dimensions.width / 4.0,
            map_dimensions.height / 4.0,
            0.0,
        )
        .with_scale(Vec3::new(0.5, 0.5, 0.0)),
        ..default()
    });

    // for x in 0..64 {
    //     for y in 0..64 {
    //         commands
    //             .spawn(SpriteBundle {
    //                 texture: water.clone(),
    //                 transform: Transform::from_xyz((x * 64) as f32, (y * 64) as f32, 0.0),
    //                 global_transform: GlobalTransform::default(),
    //                 ..Default::default()
    //             })
    //             .insert(WaterTile {});
    //     }
    // }

    println!("Finished Creating the background world");
}
