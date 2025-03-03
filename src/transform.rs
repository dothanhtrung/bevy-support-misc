pub mod fading;
pub mod movement;

use bevy::prelude::{App, Plugin};
use crate::transform::fading::FadingSupportPlugin;
use crate::transform::movement::MovementSupportPlugin;

pub struct TransformSupportPlugin;

impl Plugin for TransformSupportPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MovementSupportPlugin, FadingSupportPlugin));
    }
}
