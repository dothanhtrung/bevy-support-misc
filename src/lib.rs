// Copyright 2024 Trung Do <dothanhtrung@pm.me>

//! ### Plugin
//! doc goes here

use bevy::prelude::{Commands, Component, Entity, Query, Visibility, With};

pub mod color;
pub mod easing;
#[cfg(feature = "save")]
pub mod save;
#[cfg(feature = "setting")]
pub mod setting;
pub mod timer;
pub mod transform;
pub mod ui;

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
