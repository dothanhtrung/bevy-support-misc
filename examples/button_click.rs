use bevy::color::palettes::tailwind::{GREEN_400, GREEN_700, GREEN_800};
use bevy::prelude::*;
use bevy_support_misc::easing::scale::{ScaleEasingEffect, ScaleEasingPluginAnyState};
use bevy_support_misc::ui::button::{ButtonColorEffect, ButtonToggleEffect, ButtonTransformEffect};
use bevy_support_misc::ui::UiSupportPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Add the plugin
        .add_plugins((UiSupportPlugin, ScaleEasingPluginAnyState::new()))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn(Node {flex_direction: FlexDirection::Row, ..default() }).with_children(|parent| {
        parent.spawn((
            ButtonTransformEffect::default(),
            ButtonColorEffect::default(),
            ButtonToggleEffect {
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

        parent.spawn((
            ScaleEasingEffect::popout(Vec3::ONE, 2000),
            Button,
            BackgroundColor(GREEN_700.into()),
            Node {
                width: Val::Px(150.),
                height: Val::Px(50.),
                ..default()
            },
        )).observe(hover_in).observe(hover_out);
    });
}

fn hover_in(trigger: On<Pointer<Over>>, mut effects: Query<(&mut UiTransform, &mut ScaleEasingEffect)>) {
    if let Ok((mut transform, mut effect)) = effects.get_mut(trigger.entity) {
        if effect.on_going() {
            return;
        }
        effect.unpause();
        effect.reset();
        transform.scale = Vec2::ONE;
    }
}

fn hover_out(trigger: On<Pointer<Out>>, mut effects: Query<(&mut UiTransform, &mut ScaleEasingEffect)>) {
    info!("Hover out");
    if let Ok((mut transform, mut effect)) = effects.get_mut(trigger.entity) {
        transform.scale = Vec2::ONE;
        effect.pause();
        effect.reset();
    }
}