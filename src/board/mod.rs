use std::collections::HashMap;

use bevy::prelude::*;

use crate::{states::MainState, vectors::Vector2Int};

pub mod components;
pub mod systems;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentBoard>()
            .add_systems(OnEnter(MainState::Game), systems::spawn_map);
    }
}

#[derive(Default, Resource)]
pub struct CurrentBoard {
    pub tiles: HashMap<Vector2Int, Entity>,
    pub start: Vector2Int,
    pub exit: Vector2Int,
    pub width: i32,
    pub height: i32,
}

impl CurrentBoard {
    pub fn tile_on_board(&self, v: Vector2Int) -> bool {
        self.tiles.contains_key(&v)
    }
}
