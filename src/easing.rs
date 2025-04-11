use bevy::prelude::States;

pub mod scale;
pub mod translation;

#[derive(States, Clone, Eq, PartialEq, Debug, Hash)]
pub struct DummyState {}
