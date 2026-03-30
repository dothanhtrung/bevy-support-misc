//! ### Plugin
//! doc goes here

use bevy::prelude::{
    Commands,
    Component,
    Entity,
    Message,
    Query,
    States,
    Visibility,
    With,
};

#[cfg(feature = "bincode_loader")]
pub mod bincode_asset_loader;
pub mod camera;
pub mod color;
pub mod easing;
#[cfg(feature = "ron_loader")]
pub mod ron_asset_loader;
pub mod transform;
pub mod ui;

#[derive(States, Clone, Eq, PartialEq, Debug, Hash)]
pub struct DummyState {}

#[derive(Message)]
pub struct ShowUi<T>
where
    T: Component;

#[derive(Message)]
pub struct HideUi<T>
where
    T: Component;

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
