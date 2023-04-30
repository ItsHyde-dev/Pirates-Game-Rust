use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use bevy::{prelude::*, utils::HashMap};

use crate::utils::structs::{BlockingStructure, WaterTile};

pub fn create_water_map(commands: &mut Commands, asset_server: &AssetServer) {
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

    let file = File::open("assets/map.txt").expect("There was an error reading the map");

    for (y, line) in BufReader::new(file).lines().enumerate() {
        if let Ok(line) = line {
            for (x, c) in line.split(",").enumerate() {
                let c = c.trim();

                let current_sprite: Option<Handle<Image>> = match c {
                    index => {
                        if sprite_map.contains_key(index) {
                            Some(sprite_map[index].clone())
                        } else {
                            None
                        }
                    }
                };

                if let Some(s) = current_sprite {
                    if c != "#" {
                        commands.spawn(SpriteBundle {
                            texture: sprite_map[&"#"].clone(),
                            transform: Transform::from_xyz((x * 64) as f32, (y * 64) as f32, 0.0),
                            global_transform: GlobalTransform::default(),
                            ..Default::default()
                        });

                        commands
                            .spawn(SpriteBundle {
                                texture: s,
                                transform: Transform::from_xyz(
                                    (x * 64) as f32,
                                    (y * 64) as f32,
                                    0.0,
                                ),
                                global_transform: GlobalTransform::default(),
                                ..Default::default()
                            })
                            .insert(BlockingStructure {});
                    } else {
                        commands
                            .spawn(SpriteBundle {
                                texture: s,
                                transform: Transform::from_xyz(
                                    (x * 64) as f32,
                                    (y * 64) as f32,
                                    0.0,
                                ),
                                global_transform: GlobalTransform::default(),
                                ..Default::default()
                            })
                            .insert(WaterTile {});
                    }
                }
            }
        }
    }

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
