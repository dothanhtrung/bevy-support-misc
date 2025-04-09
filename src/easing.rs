pub mod scale;
pub mod translation;

use bevy::prelude::States;

#[derive(States, Clone, Eq, PartialEq, Debug, Hash)]
pub struct DummyState {}
