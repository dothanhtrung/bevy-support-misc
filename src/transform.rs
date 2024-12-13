use bevy::app::App;
use bevy::prelude::{Changed, Component, Plugin, Query, Res, Time, Transform, Update, Vec3};

pub struct TransformSupportPlugin;

impl Plugin for TransformSupportPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, moving);
    }
}

#[derive(Component, Default)]
pub struct Movement {
    pub velocity: f32,
    pub des: Vec<Vec3>,
}

fn moving(time: Res<Time>, mut query: Query<(&mut Transform, &mut Movement), Changed<Movement>>) {
    for (mut transform, mut movement) in query.iter_mut() {
        if movement.des.is_empty() {
            continue;
        }

        let mut arrived_x = false;
        let mut arrived_y = false;
        let v = movement.velocity * time.delta().as_millis() as f32;
        let next_stop = movement.des.first().unwrap();
        let x2x1 = next_stop.x - transform.translation.x;
        let y2y1 = next_stop.y - transform.translation.y;
        let d = (x2x1.powi(2) + y2y1.powi(2)).sqrt();
        let mut move_x = x2x1 / d * v;
        let mut move_y = y2y1 / d * v;

        if (move_x - transform.translation.x).abs() > x2x1.abs() {
            move_x = next_stop.x;
            arrived_x = true;
        }
        if (move_y - transform.translation.y).abs() > y2y1.abs() {
            move_y = next_stop.y;
            arrived_y = true;
        }

        transform.translation.x = move_x;
        transform.translation.y = move_y;

        if arrived_x && arrived_y {
            movement.des.remove(0);
        }
    }
}
