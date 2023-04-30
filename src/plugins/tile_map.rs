use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use bevy::{prelude::*, utils::HashMap};

use crate::utils::structs::{BlockingStructure, WaterTile, WindowSize};

pub fn create_water_map(
    commands: &mut Commands,
    asset_server: &AssetServer,
    window_size: &WindowSize,
) {
    println!("Started Creating the background world");

    // use when we need to generate map using resource

    let sprite_map: HashMap<&str, Handle<Image>> = HashMap::from([
        ("#", asset_server.load("water.png")),
        ("1", asset_server.load("Tiles/island 1.png")),
        ("2", asset_server.load("Tiles/island 2.png")),
        ("3", asset_server.load("Tiles/island 3.png")),
        ("4", asset_server.load("Tiles/island 4.png")),
        ("5", asset_server.load("Tiles/island 5.png")),
        ("6", asset_server.load("Tiles/island 6.png")),
        ("7", asset_server.load("Tiles/island 7.png")),
        ("8", asset_server.load("Tiles/island 8.png")),
        ("9", asset_server.load("Tiles/island 9.png")),
        ("10", asset_server.load("Tiles/island 10.png")),
        ("11", asset_server.load("Tiles/island 11.png")),
        ("12", asset_server.load("Tiles/island 12.png")),
        ("13", asset_server.load("Tiles/island 13.png")),
        ("14", asset_server.load("Tiles/island 14.png")),
        ("15", asset_server.load("Tiles/island 15.png")),
        ("16", asset_server.load("Tiles/island 16.png")),
    ]);

    let file =
        File::open("assets/map 1._Tile Layer 2.csv").expect("There was an error reading the map");

    for (y, line) in BufReader::new(file).lines().enumerate() {
        if let Ok(line) = line {
            for (x, c) in line.split(",").enumerate() {
                let c = c.trim();

                if c == "-1" {
                    commands
                        .spawn(SpriteBundle {
                            sprite: Sprite {
                                color: Color::rgba(0.0, 0.0, 5.0, 0.2),
                                custom_size: Some(Vec2::new(128.0, 128.0)),
                                ..default()
                            },
                            transform: Transform::from_xyz(
                                ((x * 64) as f32) + 32.0,
                                ((y * 64) as f32) + 32.0,
                                0.0,
                            ),
                            global_transform: GlobalTransform::default(),
                            ..default()
                        })
                        .insert(WaterTile {});
                } else {
                    commands
                        .spawn(SpriteBundle {
                            sprite: Sprite {
                                color: Color::rgba(0.0, 0.0, 0.0, 0.0),
                                custom_size: Some(Vec2::new(128.0, 128.0)),
                                ..default()
                            },
                            transform: Transform::from_xyz(
                                ((x * 64) as f32) + 32.0,
                                ((y * 64) as f32) + 32.0,
                                0.0,
                            ),
                            global_transform: GlobalTransform::default(),
                            ..default()
                        })
                        .insert(BlockingStructure {});
                }
            }
        }
    }

    let map_1 = asset_server.load("map 1.png");

    commands.spawn(SpriteBundle {
        texture: map_1,
        transform: Transform::from_xyz(window_size.width - 319.0, window_size.height - 79.0, 0.0)
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
