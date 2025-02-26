use bevy::color::palettes::tailwind::{NEUTRAL_600, ZINC_800};
use bevy::prelude::*;
use bevy_support_misc::ui::number_input::{spawn_number_input_text, NumberInputSetting};
use bevy_support_misc::ui::UiSupportPlugin;
use bevy_text_edit::TextEditPluginNoState;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((UiSupportPlugin, TextEditPluginNoState))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    let setting = NumberInputSetting {
        width: Val::Px(400.),
        height: Val::Px(60.),
        text_bg: ZINC_800.into(),
        btn_bg: NEUTRAL_600.into(),
        max: 100,
        min: -10,
        ..default()
    };

    spawn_number_input_text(&mut commands, 1, setting);
}
