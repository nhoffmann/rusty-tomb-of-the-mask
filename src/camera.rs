use bevy::prelude::*;

use crate::{
    globals::{WINDOW_HEIGHT, WINDOW_WIDTH},
    graphics::TILE_SIZE,
};

pub fn setup(mut commands: Commands) {
    let camera = Camera2d::default();
    // Place the camera above the center of the 8x8 tile grid we currently arbitrarily spawn
    // let transform = Transform::from_translation(Vec3::new(4. * TILE_SIZE, 4. * TILE_SIZE, 2.));
    let transform = Transform::from_translation(Vec3::new(
        WINDOW_WIDTH / 4. + TILE_SIZE / 2.,
        WINDOW_HEIGHT / 4. - TILE_SIZE,
        2.,
    ));

    commands.spawn((camera, transform));
}
