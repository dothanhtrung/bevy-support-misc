use bevy::app::App;
use bevy::prelude::{Component, Plugin, Query, Res, Time, Transform, Update, Vec3};

pub struct TransformSupportPlugin;

impl Plugin for TransformSupportPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, moving_2d);
    }
}

#[derive(Component, Default)]
pub struct Movement {
    pub velocity: f32,
    pub des: Vec<Vec3>,
}

fn moving_2d(time: Res<Time>, mut query: Query<(&mut Transform, &mut Movement)>) {
    for (mut transform, mut movement) in query.iter_mut() {
        if movement.des.is_empty() {
            continue;
        }

        let mut arrived_x = false;
        let mut arrived_y = false;
        let v = movement.velocity * (time.delta().as_millis() as f32);
        let next_stop = movement.des.first().unwrap();
        let x2x1 = next_stop.x - transform.translation.x;
        let y2y1 = next_stop.y - transform.translation.y;
        let d = (x2x1.powi(2) + y2y1.powi(2)).sqrt();
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

        if arrived_x && arrived_y {
            movement.des.remove(0);
        }
    }
}
