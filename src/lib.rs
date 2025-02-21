// Copyright 2024 Trung Do <dothanhtrung@pm.me>

//! ### Plugin
//! doc goes here

use bevy::prelude::{Commands, Component, DespawnRecursiveExt, Entity, Query, Visibility, With};

pub mod easing;
pub mod save;
pub mod setting;
pub mod transform;
pub mod ui;
pub mod color;
pub mod timer;

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
        commands.entity(e).despawn_recursive();
    }
}
