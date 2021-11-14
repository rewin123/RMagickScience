use bevy::prelude::*;

fn main() {
   App::build()
        .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.9)))
        .add_plugins(DefaultPlugins)
        .run();
}
