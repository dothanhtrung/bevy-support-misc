use bevy::app::App;
use bevy::prelude::{
    default, Component, Curve, Deref, DerefMut, EaseFunction, EasingCurve, Entity, Event, EventWriter, Plugin, Quat,
    Query, Res, Time, Transform, Update, Vec3,
};

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
    transform_gap: Transform,
    transform_orig: Option<Transform>,
    transform_start: Option<Transform>,
    duration_ms: u128,
    elapsed: u128,
    function: Option<EaseFunction>,
}

impl EasingEffect {
    pub fn orig_transform(&self) -> Option<Transform> {
        self.transform_orig
    }

    pub fn popout(scale_gap: Vec3, duration_ms: u128) -> Self {
        Self {
            transform_gap: Transform::from_scale(scale_gap),
            duration_ms,
            elapsed: 0,
            function: Some(EaseFunction::ElasticOut),
            ..default()
        }
    }

    pub fn shake(translation_gap: Vec3, origin_translation: Vec3, duration_ms: u128) -> Self {
        Self {
            transform_gap: Transform::from_translation(translation_gap),
            transform_orig: Some(Transform::from_translation(origin_translation)),
            transform_start: Some(Transform::from_translation(origin_translation - translation_gap)),
            duration_ms,
            elapsed: 0,
            function: Some(EaseFunction::Elastic(50.)),
        }
    }

    pub fn with_duration(mut self, duration_ms: u128) -> Self {
        self.duration_ms = duration_ms;
        self
    }

    pub fn with_scale_gap(mut self, scale: Vec3) -> Self {
        self.transform_gap.scale = scale;
        self
    }

    pub fn with_translation_gap(mut self, translation: Vec3) -> Self {
        self.transform_gap.translation = translation;
        self
    }

    pub fn with_rotation_gap(mut self, rotation: Quat) -> Self {
        self.transform_gap.rotation = rotation;
        self
    }

    pub fn with_ease_function(mut self, ease_function: EaseFunction) -> Self {
        self.function = Some(ease_function);
        self
    }
}

fn easing(
    mut query: Query<(&mut Transform, &mut EasingEffect, Entity)>,
    mut event: EventWriter<EasingEnded>,
    time: Res<Time>,
) {
    let delta = time.delta().as_millis();
    for (mut transform, mut easing, entity) in query.iter_mut() {
        if easing.elapsed > easing.duration_ms || easing.function.is_none() {
            continue;
        }

        let f = EasingCurve::new(0.0, 1.0, easing.function.unwrap());
        if easing.transform_orig.is_none() {
            easing.transform_orig = Some(*transform);
        }
        if easing.transform_start.is_none() {
            easing.transform_start = easing.transform_orig;
        }

        easing.elapsed += delta;
        let percent = easing.elapsed as f32 / easing.duration_ms as f32;
        let rate = f.sample(percent).unwrap_or(1.);
        if let Some(start) = easing.transform_start {
            transform.scale = start.scale + easing.transform_gap.scale * Vec3::splat(rate);
            transform.translation = start.translation + easing.transform_gap.translation * Vec3::splat(rate);
        }

        if percent >= 1. {
            event.send(EasingEnded(entity));
        }
    }
}
