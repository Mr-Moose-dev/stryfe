use bevy::prelude::*;

pub struct Stryfe;

impl Plugin for Stryfe {
    fn build(&self, app: &mut App) {

    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, Stryfe))
        .run();
}


