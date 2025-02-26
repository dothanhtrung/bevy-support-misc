// Copyright 2024 Trung Do <dothanhtrung@pm.me>

pub mod button;
pub mod number_input;

use crate::ui::button::GameButtonPlugin;
use bevy::prelude::With;
use bevy::{
    app::{App, Plugin, Update},
    prelude::{Component, Query, Text2d, Transform},
};

pub struct UiSupportPlugin;

impl Plugin for UiSupportPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GameButtonPlugin).add_systems(Update, update_debug_info);
    }
}

#[derive(Component, Default)]
#[require(Text2d)]
pub struct TransformInfo;

pub fn update_debug_info(mut query: Query<(&mut Text2d, &Transform), With<TransformInfo>>) {
    for (mut text, transform) in query.iter_mut() {
        **text = format!("{:?}", transform.translation);
    }
}
