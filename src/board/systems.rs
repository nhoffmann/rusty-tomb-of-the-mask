use bevy::prelude::*;
use std::collections::HashMap;

use crate::{board::components::Exit, vectors::Vector2Int};

use super::{
    components::{Position, Tile},
    CurrentBoard,
};

pub fn spawn_map(mut commands: Commands, mut current: ResMut<CurrentBoard>) {
    info!("Spawning map");

    current.tiles = HashMap::new();

    // Spawn a grid of tiles of arbitrary size for now
    for x in 0..5 {
        for y in 0..4 {
            let v = Vector2Int::new(x, y);
            let tile = commands.spawn((Position { v }, Tile)).id();
            current.tiles.insert(v, tile);
        }
    }

    let remove_vector = &Vector2Int::new(1, 2);
    let tile_entity = current.tiles.get(remove_vector).unwrap();
    commands.entity(*tile_entity).despawn();
    current.tiles.remove(remove_vector);

    let remove_vector = &Vector2Int::new(4, 3);
    let tile_entity = current.tiles.get(remove_vector).unwrap();
    commands.entity(*tile_entity).despawn();
    current.tiles.remove(remove_vector);

    let exit_vector = Vector2Int::new(2, 1);
    let exit: Entity = commands.spawn((Position { v: exit_vector }, Exit)).id();
    current.tiles.insert(exit_vector, exit);
}
