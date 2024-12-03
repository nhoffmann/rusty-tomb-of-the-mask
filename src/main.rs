use bevy::prelude::*;

use tomb_of_the_mask::TombOfTheMask;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TombOfTheMask)
        .run();
}
