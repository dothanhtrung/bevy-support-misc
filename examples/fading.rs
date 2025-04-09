use bevy::color::palettes::tailwind::GREEN_400;
use bevy::prelude::*;
use bevy_support_misc::easing::scale::{ScaleEasingEffect, ScaleEasingPluginAnyState};
use bevy_support_misc::transform::fading::{Fade, FadeSupport};
use bevy_support_misc::transform::TransformSupportPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((TransformSupportPlugin, ScaleEasingPluginAnyState::new()))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Button,
        Node {
            width: Val::Px(150.),
            height: Val::Px(50.),
            justify_self: JustifySelf::Center,
            align_self: AlignSelf::Center,
            ..default()
        },
        BackgroundColor(GREEN_400.into()),
        Text::new("Button"),
        FadeSupport::new(Timer::from_seconds(1., TimerMode::Once), Fade::Out, true),
        ScaleEasingEffect::popout(Vec3::splat(2.), 1000),
    ));
}
