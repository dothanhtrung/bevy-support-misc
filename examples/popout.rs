use bevy::color::palettes::tailwind::GREEN_400;
use bevy::prelude::*;
use bevy_support_misc::easing::scale::{ScaleEasingEffect, ScaleEasingPluginAnyState};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ScaleEasingPluginAnyState::new())
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands
        .spawn((
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
        ))
        .observe(hover_in)
        .observe(hover_out);
}

fn hover_in(trigger: On<Pointer<Over>>, mut commands: Commands) {
    commands
        .entity(trigger.entity)
        .insert(ScaleEasingEffect::popout(Vec3::splat(2.), 1000));
}

fn hover_out(
    trigger: On<Pointer<Out>>,
    mut commands: Commands,
    mut query: Query<(&mut Transform, &ScaleEasingEffect)>,
) {
    if let Ok((mut transform, popout)) = query.get_mut(trigger.entity) {
        transform.scale = popout.orig_scale().unwrap_or(Vec3::splat(1.));
    }
    commands.entity(trigger.entity).remove::<ScaleEasingEffect>();
}
