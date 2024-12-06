use bevy::prelude::*;

use crate::{board::components::Position, pieces::components::Piece};

use super::assets::{AsciiSpriteSheet, ATLAS_PATH, SPRITE_PLAYER, SPRITE_QUESTION_MARK};
use super::{PIECE_SPEED, PIECE_Z, POSITION_TOLERANCE, TILE_SIZE};

pub fn update_piece_position(
    mut query: Query<(&Position, &mut Transform), With<Piece>>,
    time: Res<Time>,
) {
    for (position, mut transform) in query.iter_mut() {
        let target = super::get_world_position(position, PIECE_Z);
        let d = (target - transform.translation).length();
        if d > POSITION_TOLERANCE {
            transform.translation = transform
                .translation
                .lerp(target, PIECE_SPEED * time.delta_secs());
        } else {
            transform.translation = target;
        }
    }
}

pub fn spawn_piece_renderer(
    mut commands: Commands,
    query: Query<(Entity, &Position, &Piece), Added<Piece>>,
    asset_server: Res<AssetServer>,
    sprite_atlas: Res<AsciiSpriteSheet>,
) {
    // TODO Load the sprite sheet only once
    let ascii_sprite: Handle<Image> = asset_server.load(ATLAS_PATH);

    for (entity, position, piece) in query.iter() {
        let sprite_idx = match piece.kind.as_str() {
            "Player" => SPRITE_PLAYER,
            _ => SPRITE_QUESTION_MARK,
        };

        let texture = TextureAtlas {
            layout: sprite_atlas.0.clone(),
            index: sprite_idx,
        };

        let v = super::get_world_position(position, PIECE_Z);

        commands.entity(entity).insert((
            Sprite {
                image: ascii_sprite.clone(),
                texture_atlas: Some(texture),
                color: Color::WHITE,
                custom_size: Some(Vec2::splat(TILE_SIZE)),
                ..Default::default()
            },
            Transform::from_translation(v),
        ));

        info!("Rendered piece {:?} at {:?}", piece.kind, v);
    }
}
