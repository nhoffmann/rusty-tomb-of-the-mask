use bevy::prelude::*;

use crate::{
    board::components::{Position, Tile},
    graphics::{
        assets::{ATLAS_PATH, SPRITE_FLOOR},
        TILE_Z,
    },
};

use super::{assets::AsciiSpriteSheet, TILE_SIZE};

pub fn spawn_tile_renderer(
    mut commands: Commands,
    query: Query<(Entity, &Position), Added<Tile>>,
    asset_server: Res<AssetServer>,
    sprite_atlas: Res<AsciiSpriteSheet>,
) {
    let ascii_sprite: Handle<Image> = asset_server.load(ATLAS_PATH);

    info!("Spawning tile renderer");

    for (entity, position) in query.iter() {
        let texture = TextureAtlas {
            layout: sprite_atlas.0.clone(),
            index: SPRITE_FLOOR,
        };
        let v = super::get_world_position(position, TILE_Z);

        commands.entity(entity).insert((
            Sprite {
                image: ascii_sprite.clone(),
                texture_atlas: Some(texture),
                color: Color::srgb(1.0, 0.0, 1.0),
                custom_size: Some(Vec2::splat(TILE_SIZE)),
                ..Default::default()
            },
            Transform::from_translation(v),
        ));
        debug!("Rendered tile at {:?}", v);
    }
}
