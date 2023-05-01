use bevy::prelude::*;

#[derive(Resource)]
pub struct WindowSize {
    pub height: f32,
    pub width: f32,
}

#[derive(Component)]
pub struct WaterTile {}

#[derive(Component)]

pub struct MainCamera {}

#[derive(Component)]

pub struct BlockingStructure {}

#[derive(Resource, Clone)]
pub struct MapAssets {
    pub map_1: Handle<Image>,
}

#[derive(Clone, States, Default, PartialEq, Debug, Eq, Hash)]
pub enum GameState {
    #[default]
    Loading,
    Playing,
}
