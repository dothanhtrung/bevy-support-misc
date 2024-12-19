use bevy::app::App;
use bevy::prelude::{Component, Plugin, Query, Res, Time, Transform, Update, Vec3};

pub struct TransformSupportPlugin;

impl Plugin for TransformSupportPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, moving_2d);
    }
}

#[derive(Default)]
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
}

fn moving_2d(time: Res<Time>, mut query: Query<(&mut Transform, &mut Movement)>) {
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
        let v = velocity * (time.delta().as_millis() as f32);
        let next_stop = movement.des.first().unwrap().pos;
        let x2x1 = next_stop.x - transform.translation.x;
        let y2y1 = next_stop.y - transform.translation.y;
        let d = (x2x1.powi(2) + y2y1.powi(2)).sqrt();

        if d * v != 0. {
            let move_x = x2x1 / d * v;
            let move_y = y2y1 / d * v;

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
        }

        if x2x1 == 0. && y2y1 == 0. {
            arrived_x = true;
            arrived_y = true;
        }

        if arrived_x && arrived_y {
            movement.des.remove(0);
        }
    }
}
