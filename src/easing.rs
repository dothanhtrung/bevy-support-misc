pub mod scale;
pub mod translation;

use crate::easing::scale::ScaleEasingPlugin;
use crate::easing::translation::TranslationEasingPlugin;
use bevy::app::App;
use bevy::prelude::Plugin;

pub struct EasingSupportPlugin;

impl Plugin for EasingSupportPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ScaleEasingPlugin, TranslationEasingPlugin));
    }
}
