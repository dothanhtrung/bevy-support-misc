use bevy::app::App;
use bevy::prelude::{Component, Plugin, Query, Res, Time, Transform, Update, Vec3};

pub struct TransformSupportPlugin;

impl Plugin for TransformSupportPlugin {
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
}

fn travel(time: Res<Time>, mut query: Query<(&mut Transform, &mut Movement)>) {
    for (mut transform, mut movement) in query.iter_mut() {
        if movement.des.is_empty() {
            continue;
        }

        let des = movement.des.first().unwrap();
        let velocity = if let Some(custom_v) = des.custom_velocity {
            custom_v
        } else {
            movement.velocity
        };

        let mut arrived_x = false;
        let mut arrived_y = false;
        let mut arrived_z = false;
        let v = velocity * (time.delta().as_millis() as f32);
        let next_stop = movement.des.first().unwrap().pos;
        let x2x1 = next_stop.x - transform.translation.x;
        let y2y1 = next_stop.y - transform.translation.y;
        let z2z1 = next_stop.z - transform.translation.z;

        let d_yz = (y2y1.powi(2) + z2z1.powi(2)).sqrt();
        let d = (d_yz.powi(2) + x2x1.powi(2)).sqrt();

        if d * v != 0. {
            let move_x = x2x1 / d * v;
            let move_y = y2y1 / d * v;
            let move_z = z2z1 / d * v;

            if move_x.abs() >= x2x1.abs() {
                transform.translation.x = next_stop.x;
                arrived_x = true;
            } else {
                transform.translation.x += move_x;
            }

            if move_y.abs() >= y2y1.abs() {
                transform.translation.y = next_stop.y;
                arrived_y = true;
            } else {
                transform.translation.y += move_y;
            }

            if move_z.abs() >= z2z1.abs() {
                transform.translation.z = next_stop.z;
                arrived_z = true;
            } else {
                transform.translation.z += move_z;
            }
        }

        if arrived_x && arrived_y && arrived_z {
            if movement.circle {
                let first_des = movement.as_ref().des.first().unwrap().clone();
                movement.des.push(first_des);
            }
            movement.des.remove(0);
        }
    }
}
