use bevy::color::palettes::tailwind::{GREEN_400, GREEN_800};
use bevy::prelude::*;
use bevy_support_misc::ui::button::{ButtonColorEffect, ButtonToggleEffect, ButtonTransformEffect};
use bevy_support_misc::ui::UiSupportPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Add the plugin
        .add_plugins(UiSupportPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        ButtonTransformEffect::default(),
        ButtonColorEffect::default(),
        ButtonToggleEffect{
            on_color: GREEN_400.into(),
            off_color: GREEN_800.into(),
            enabled: true,
            ..default()
        },
        Button,
        Node {
            width: Val::Px(150.),
            height: Val::Px(50.),
            ..default()
        },
        BackgroundColor(GREEN_400.into()),
        Text::new("On"),
    ));
}
