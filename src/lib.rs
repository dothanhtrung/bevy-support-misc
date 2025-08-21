//! ### Plugin
//! doc goes here

use bevy::prelude::{Commands, Component, Entity, Query, States, Visibility, With};

pub mod color;
pub mod easing;
#[cfg(feature = "save")]
pub mod save;
#[cfg(feature = "setting")]
pub mod setting;
pub mod transform;
pub mod ui;
#[cfg(feature = "ron_loader")]
pub mod ron_asset_loader;
pub mod camera;

#[derive(States, Clone, Eq, PartialEq, Debug, Hash)]
struct DummyState {}

pub fn show_ui<T>(mut query: Query<&mut Visibility, With<T>>)
where
    T: Component,
{
    for mut visibility in query.iter_mut() {
        *visibility = Visibility::Visible;
    }
}

pub fn hide_ui<T>(mut query: Query<&mut Visibility, With<T>>)
where
    T: Component,
{
    for mut visibility in query.iter_mut() {
        *visibility = Visibility::Hidden;
    }
}

pub fn despawn<T>(mut commands: Commands, query: Query<Entity, With<T>>)
where
    T: Component,
{
    for e in query.iter() {
        commands.entity(e).despawn();
    }
}
