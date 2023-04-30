use bevy::prelude::*;
// use std::io::BufRead;
// use std::{fs::File, io::BufReader};

// use crate::utils::structs::CharsetAsset;

// pub struct TileMapPlugin;

// impl Plugin for TileMapPlugin {
//     // fn build(&self, app: &mut App) {
//     //     app.add_startup_system(create_water_map);
//     // }
// }

pub fn create_water_map(commands: &mut Commands, water: Handle<Image>) {
    println!("Started Creating the background world");

    // use when we need to generate map using resource

    // let file = File::open("assets/map.txt").expect("There was an error reading the map");

    // for (y, line) in BufReader::new(file).lines().enumerate() {
    //     if let Ok(line) = line {
    //         for (x, _) in line.chars().enumerate() {
    //             commands.spawn(SpriteBundle {
    //                 texture: water.clone(),
    //                 transform: Transform::from_xyz((x * 64) as f32, (y * 64) as f32, 0.0),
    //                 global_transform: GlobalTransform::default(),
    //                 ..Default::default()
    //             });
    //         }
    //     }
    // }

    for x in 0..64 {
        for y in 0..64 {
            commands.spawn(SpriteBundle {
                texture: water.clone(),
                transform: Transform::from_xyz((x * 64) as f32, (y * 64) as f32, 0.0),
                global_transform: GlobalTransform::default(),
                ..Default::default()
            });
        }
    }

    println!("Finished Creating the background world");
}
