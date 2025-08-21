use bevy::app::App;
use bevy::prelude::{
    in_state, Camera, Component, IntoScheduleConfigs, Plugin, Query, States, Transform, Update, Vec3, With, Without,
};

pub struct CameraSupportPlugin<T>
where
    T: States,
{
    pub states: Option<Vec<T>>,
}

impl<T> CameraSupportPlugin<T>
where
    T: States,
{
    pub fn new(states: Vec<T>) -> Self {
        Self { states: Some(states) }
    }

    pub fn any() -> Self {
        Self { states: None }
    }
}

impl<T> Plugin for CameraSupportPlugin<T>
where
    T: States,
{
    fn build(&self, app: &mut App) {
        if let Some(states) = &self.states {
            for state in states {
                app.add_systems(Update, lock_target.run_if(in_state(state)));
            }
        } else {
            app.add_systems(Update, lock_target);
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
