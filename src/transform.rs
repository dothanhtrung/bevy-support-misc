pub mod fading;

use crate::transform::fading::FadingSupportPlugin;
use bevy::prelude::{App, Plugin};

pub struct TransformSupportPlugin;

impl Plugin for TransformSupportPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FadingSupportPlugin);
    }
}
