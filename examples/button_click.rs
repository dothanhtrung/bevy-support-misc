use bevy::color::palettes::tailwind::GREEN_400;
use bevy::prelude::*;
use bevy_support_misc::ui::button::ButtonEffect;
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
    commands.spawn(Camera2d::default());

    commands.spawn((
        ButtonEffect::default(),
        Button,
        Node {
            width: Val::Px(150.),
            height: Val::Px(50.),
            ..default()
        },
        BackgroundColor(GREEN_400.into()),
        Text::new("Button"),
    ));
}
