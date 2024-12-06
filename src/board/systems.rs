use bevy::prelude::*;
use std::collections::HashMap;

use crate::vectors::Vector2Int;

use super::{
    components::{Position, Tile},
    CurrentBoard,
};

pub fn spawn_map(mut commands: Commands, mut current: ResMut<CurrentBoard>) {
    info!("Spawning map");

    current.tiles = HashMap::new();

    // Spawn a grid of tiles of arbitrary size for now
    for x in 0..8 {
        for y in 0..8 {
            let v = Vector2Int::new(x, y);
            let tile = commands.spawn((Position { v }, Tile)).id();
            current.tiles.insert(v, tile);
        }
    }
}
