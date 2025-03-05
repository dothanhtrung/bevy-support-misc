use bevy::app::{App, Update};
use bevy::math::Vec3;
use bevy::prelude::{Commands, Component, Entity, Event, Plugin, Query, Res, Time, Transform};

pub struct MovementSupportPlugin;

impl Plugin for MovementSupportPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, travel);
    }
}

#[derive(Default, Clone)]
pub struct Destination {
    pub pos: Vec3,
    pub custom_velocity: Option<f32>,
}

impl Destination {
    pub fn from_pos(pos: Vec3) -> Self {
        Self { pos, ..Self::default() }
    }
}

#[derive(Component, Default)]
pub struct Movement {
    pub velocity: f32,
    pub des: Vec<Destination>,
    pub circle: bool,
    pub is_freezed: bool,
}

impl Movement {
    pub fn freeze(&mut self) {
        self.is_freezed = true;
    }

    pub fn go(&mut self) {
        self.is_freezed = false;
    }
}

#[derive(Event)]
pub struct Arrived;

pub fn travel(mut commands: Commands, time: Res<Time>, mut query: Query<(&mut Transform, &mut Movement, Entity)>) {
    for (mut transform, mut movement, e) in query.iter_mut() {
        if movement.des.is_empty() || movement.is_freezed {
            continue;
        }

        let des = movement.des.first().unwrap();
        let velocity = if let Some(custom_v) = des.custom_velocity {
            custom_v
        } else {
            movement.velocity
        };

        let v = velocity * (time.delta().as_millis() as f32);
        let next_stop = movement.des.first().unwrap().pos;
        let x2x1 = next_stop.x - transform.translation.x;
        let y2y1 = next_stop.y - transform.translation.y;
        let _z2z1 = next_stop.z - transform.translation.z;

        let mut arrived_x = x2x1 == 0.;
        let mut arrived_y = y2y1 == 0.;
        #[cfg(feature = "3d")]
        let mut arrived_z = _z2z1 == 0.;

        #[cfg(feature = "3d")]
        let d_yz_pow2 = y2y1.powi(2) + _z2z1.powi(2);
        #[cfg(not(feature = "3d"))]
        let d_yz_pow2 = y2y1.powi(2);

        let d = (d_yz_pow2 + x2x1.powi(2)).sqrt();

        if d != 0. {
            let move_x = x2x1 / d * v;
            let move_y = y2y1 / d * v;
            let _move_z = _z2z1 / d * v;

            if !arrived_x {
                if move_x.abs() >= x2x1.abs() {
                    transform.translation.x = next_stop.x;
                    arrived_x = true;
                } else {
                    transform.translation.x += move_x;
                }
            }

            if !arrived_y {
                if move_y.abs() >= y2y1.abs() {
                    transform.translation.y = next_stop.y;
                    arrived_y = true;
                } else {
                    transform.translation.y += move_y;
                }
            }

            #[cfg(feature = "3d")]
            if !arrived_z {
                if _move_z.abs() >= _z2z1.abs() {
                    transform.translation.z = next_stop.z;
                    arrived_z = true;
                } else {
                    transform.translation.z += _move_z;
                }
            }
        }

        #[cfg(feature = "3d")]
        let arrived = arrived_x && arrived_y && arrived_z;

        #[cfg(not(feature = "3d"))]
        let arrived = arrived_x && arrived_y;

        if arrived {
            commands.trigger_targets(Arrived, e);
            if movement.circle {
                let first_des = movement.as_ref().des.first().unwrap().clone();
                movement.des.push(first_des);
            }
            movement.des.remove(0);
        }
    }
}
