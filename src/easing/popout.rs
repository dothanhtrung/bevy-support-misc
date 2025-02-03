use bevy::app::App;
use bevy::prelude::{Component, Curve, EaseFunction, EasingCurve, Plugin, Query, Res, Time, Transform, Update, Vec3};

pub struct PopoutSupportPlugin;

impl Plugin for PopoutSupportPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, popout);
    }
}

#[derive(Component)]
pub struct PopoutEffect {
    target_scale: Vec3,
    orig_scale: Option<Vec3>,
    duration_ms: u128,
    elapsed: u128,
}

impl PopoutEffect {
    pub fn new(target_scale: Vec3, duration_ms: u128) -> Self {
        Self {
            target_scale,
            duration_ms,
            orig_scale: None,
            elapsed: 0,
        }
    }

    pub fn orig_scale(&self) -> Option<Vec3> {
        self.orig_scale
    }
}

fn popout(mut query: Query<(&mut Transform, &mut PopoutEffect)>, time: Res<Time>) {
    let delta = time.delta().as_millis();
    let f = EasingCurve::new(0.0, 1.0, EaseFunction::ElasticOut);
    for (mut transform, mut popout) in query.iter_mut() {
        if popout.orig_scale.is_none() {
            popout.orig_scale = Some(transform.scale);
        }

        if popout.elapsed > popout.duration_ms {
            continue;
        }

        popout.elapsed += delta;
        let percent = popout.elapsed as f32 / popout.duration_ms as f32;
        let rate = f.sample(percent).unwrap_or(1.);
        let gap = popout.target_scale - popout.orig_scale.unwrap();
        transform.scale = popout.orig_scale.unwrap() + gap * Vec3::splat(rate);
    }
}
