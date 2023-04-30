use bevy::{prelude::*, sprite::collide_aabb::collide};

use crate::utils::structs::{BlockingStructure, MainCamera, WindowSize};
#[derive(Component)]
pub struct Ship {
    pub direction: Direction,
}

#[derive(Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Ship {
    pub fn spawn_player(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        window_size: WindowSize,
    ) {
        let player_sprite: Handle<Image> = asset_server.load("ship.png");

        commands.spawn((
            SpriteBundle {
                texture: player_sprite,
                transform: Transform::from_xyz(
                    (window_size.width / 2.0),
                    (window_size.height / 2.0),
                    0.0,
                )
                .with_rotation(Quat::from_rotation_z((180.0_f32).to_radians())),
                ..default()
            },
            Ship {
                direction: Direction::Up,
            },
        ));

        println!("Player spawned");
    }

    pub fn move_ship(
        kb: Res<Input<KeyCode>>,
        mut ship_query: Query<(&mut Ship, &mut Transform), With<Ship>>,
        time: Res<Time>,
        // window: Res<WindowSize>,
        mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<Ship>)>,
        wall_query: Query<
            &Transform,
            (With<BlockingStructure>, Without<Ship>, Without<MainCamera>),
        >,
    ) {
        let mut movement_direction = Vec3::ZERO;
        let mut rotation: Direction;

        const SHIP_SPEED: f32 = 256.0;

        if let Ok((mut ship, mut transform)) = ship_query.get_single_mut() {
            rotation = ship.direction.clone();

            if kb.pressed(KeyCode::Right) {
                movement_direction = Vec3::new(1.0, 0.0, 0.0);
                rotation = Direction::Right;
            }
            if kb.pressed(KeyCode::Left) {
                movement_direction = Vec3::new(-1.0, 0.0, 0.0);
                rotation = Direction::Left;
            }
            if kb.pressed(KeyCode::Up) {
                movement_direction = Vec3::new(0.0, 1.0, 0.0);
                rotation = Direction::Up;
            }
            if kb.pressed(KeyCode::Down) {
                movement_direction = Vec3::new(0.0, -1.0, 0.0);
                rotation = Direction::Down;
            }

            if movement_direction.length() > 0.0 {
                movement_direction = movement_direction.normalize();
            }

            let moved = movement_direction * SHIP_SPEED * time.delta_seconds();

            let collision = Self::check_collision(transform.translation + moved, &wall_query);

            if collision {
                return;
            }

            transform.translation += moved;
            if let Ok(mut camera_transform) = camera_query.get_single_mut() {
                camera_transform.translation += moved;
            }
            let total_rotation = Self::get_rad_from_directions(&rotation, &ship.direction);
            transform.rotate_z(total_rotation);
            ship.direction = rotation;
        }
    }

    fn get_rad_from_directions(first: &Direction, second: &Direction) -> f32 {
        let f = match first {
            Direction::Up => 1,
            Direction::Left => 2,
            Direction::Down => 3,
            Direction::Right => 4,
        };

        let s = match second {
            Direction::Up => 1,
            Direction::Left => 2,
            Direction::Down => 3,
            Direction::Right => 4,
        };

        return (((f - s) * 90) as f32).to_radians();
    }

    fn check_collision(
        player_position: Vec3,
        wall_query: &Query<
            &Transform,
            (With<BlockingStructure>, Without<Ship>, Without<MainCamera>),
        >,
    ) -> bool {
        for wall_transform in wall_query.iter() {
            let collision = collide(
                player_position,
                Vec2::new(64.0, 64.0),
                wall_transform.translation,
                Vec2::new(64.0, 64.0),
            );

            if collision.is_some() {
                return true;
            }
        }

        return false;
    }
}
