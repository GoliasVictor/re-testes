//! This example demonstrates Bevy's immediate mode drawing API intended for visual debugging.


use bevy::prelude::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup,spawn_cam)
    .run();
}

fn spawn_cam(mut commands: Commands){
    commands.spawn(Camera2dBundle::default());
}