use crate::DummyState;
use bevy::app::App;
use bevy::prelude::{
    in_state, Camera, Component, IntoScheduleConfigs, Plugin, Query, States, Transform, Update, Vec3, With, Without,
};

macro_rules! plugin_systems {
    ( ) => {
        (lock_target)
    };
}

pub struct CameraSupportPlugin<T>
where
    T: States,
{
    pub states: Vec<T>,
}

impl<T> CameraSupportPlugin<T>
where
    T: States,
{
    pub fn new(states: Vec<T>) -> Self {
        Self { states }
    }

    pub fn any() -> Self {
        Self { states: Vec::new() }
    }
}

pub struct CameraSupportPluginAnyState;

impl CameraSupportPluginAnyState {
    pub fn any() -> CameraSupportPlugin<DummyState> {
        CameraSupportPlugin::new(Vec::new())
    }
}

impl<T> Plugin for CameraSupportPlugin<T>
where
    T: States,
{
    fn build(&self, app: &mut App) {
        if self.states.is_empty() {
            app.add_systems(Update, plugin_systems!());
        } else {
            for state in &self.states {
                app.add_systems(Update, plugin_systems!().run_if(in_state(state.clone())));
            }
        }
    }
}

#[derive(Component, Default)]
pub struct CameraLock {
    pub rel_pos: Vec3,
    pub is_free: bool,
}

fn lock_target(
    target: Query<(&Transform, &CameraLock)>,
    mut camera: Query<&mut Transform, (With<Camera>, Without<CameraLock>)>,
) {
    if let Ok((target_transform, camera_lock)) = target.single() {
        if camera_lock.is_free {
            return;
        }

        for mut cam_transform in camera.iter_mut() {
            cam_transform.translation = target_transform.translation + camera_lock.rel_pos;
        }
    }
}
