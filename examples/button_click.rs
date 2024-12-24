use bevy::color::palettes::tailwind::GRAY_400;
use bevy::prelude::*;
use bevy_support_misc::ui::{PressEffect, UiSupportPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Add the plugin
        .add_plugins(UiSupportPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());

    commands.spawn((
        PressEffect::default(),
        Button,
        Node {
            width: Val::Px(150.),
            height: Val::Px(50.),
            ..default()
        },
        BackgroundColor(GRAY_400.into()),
    ));
}
