use bevy::prelude::*;

pub const ATLAS_PATH: &str = "ascii.png";

#[derive(Resource)]
pub struct AsciiSpriteSheet(pub Handle<TextureAtlasLayout>);

pub const SPRITE_FLOOR: usize = 177;
pub const SPRITE_PLAYER: usize = 1;
pub const SPRITE_QUESTION_MARK: usize = 63;

impl FromWorld for AsciiSpriteSheet {
    fn from_world(world: &mut World) -> Self {
        let texture_atlas = TextureAtlasLayout::from_grid(UVec2::splat(10), 16, 16, None, None);

        let mut texture_atlases = world
            .get_resource_mut::<Assets<TextureAtlasLayout>>()
            .unwrap();
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        Self(texture_atlas_handle)
    }
}
