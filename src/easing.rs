
use bevy::app::App;
use bevy::prelude::{default, Component, Curve, Deref, DerefMut, EaseFunction, EasingCurve, Entity, Event, Plugin, Query, Res, Time, Transform, Update, Vec3};

pub struct EasingSupportPlugin;

impl Plugin for EasingSupportPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EasingEnded>().add_systems(Update, easing);
    }
}

#[derive(Event, Deref, DerefMut)]
pub struct EasingEnded(pub Entity);

#[derive(Component, Default)]
pub struct EasingEffect {
    target_transform: Transform,
    orig_transform: Option<Transform>,
    start_transform:  Option<Transform>,
    duration_ms: u128,
    elapsed: u128,
}

impl EasingEffect {

    pub fn orig_transform(&self) -> Option<Transform> {
        self.orig_transform
    }

    pub fn popout(scale: Vec3, duration_ms: u128) -> Self {
        Self {
            target_transform: Transform::from_scale(scale),
            duration_ms,
            elapsed: 0,
            ..default()
        }
    }

    pub fn shake(translation: Vec3, duration_ms: u128) -> Self {
        Self {
            target_transform: Transform::from_translation(translation),
            duration_ms,
            elapsed: 0,
            ..default()
        }
    }

    pub fn with_duration(mut self, duration_ms: u128) -> Self {
        self.duration_ms = duration_ms;
        self
    }

    pub fn with_scale(mut self, scale: Vec3) -> Self {
        self.target_transform.scale = scale;
        self
    }

    pub fn with_translation(mut self, translattion: Vec3) -> Self {
        self.target_transform.translation = translattion;
        self
    }
}

fn easing(mut query: Query<(&mut Transform, &mut EasingEffect)>, time: Res<Time>) {
    let delta = time.delta().as_millis();
    let f = EasingCurve::new(0.0, 1.0, EaseFunction::ElasticOut);
    for (mut transform, mut popout) in query.iter_mut() {
        if popout.orig_transform.is_none() {
            popout.orig_transform = Some(*transform);
        }
        if popout.start_transform.is_none() {
            popout.start_transform = popout.orig_transform;
        }

        if popout.elapsed > popout.duration_ms {
            continue;
        }

        popout.elapsed += delta;
        let percent = popout.elapsed as f32 / popout.duration_ms as f32;
        let rate = f.sample(percent).unwrap_or(1.);
        if let Some(start) = popout.start_transform {
            let scale_gap = popout.target_transform.scale - start.scale;
            let translation_gap = popout.target_transform.translation - start.translation;

            transform.scale = start.scale + scale_gap * Vec3::splat(rate);
            transform.translation = start.translation + translation_gap * Vec3::splat(rate);
        }
    }
}
