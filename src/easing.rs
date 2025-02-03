pub mod popout;

pub use popout::*;
use bevy::app::App;
use bevy::prelude::Plugin;
use popout::PopoutSupportPlugin;

pub struct EasingSupportPlugin;

impl Plugin for EasingSupportPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PopoutSupportPlugin);
    }
}