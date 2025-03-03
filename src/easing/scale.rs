use bevy::app::{App, Update};
use bevy::math::Vec3;
use bevy::prelude::{
    default, Commands, Component, Curve, Deref, DerefMut, EaseFunction, EasingCurve, Entity, Event, EventWriter,
    Plugin, Query, Res, Time, Transform,
};

pub struct ScaleEasingPlugin;

impl Plugin for ScaleEasingPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ScaleEasingEnded>().add_systems(Update, easing);
    }
}

#[derive(Event, Deref, DerefMut)]
pub struct ScaleEasingEnded(pub Entity);

#[derive(Component, Default)]
pub struct ScaleEasingEffect {
    scale_gap: Vec3,
    scale_orig: Option<Vec3>,
    scale_start: Option<Vec3>,
    duration_ms: u128,
    elapsed: u128,
    function: Option<EaseFunction>,
}

impl ScaleEasingEffect {
    pub fn orig_scale(&self) -> Option<Vec3> {
        self.scale_orig
    }

    pub fn popout(scale_gap: Vec3, duration_ms: u128) -> Self {
        Self {
            scale_gap,
            duration_ms,
            elapsed: 0,
            function: Some(EaseFunction::ElasticOut),
            ..default()
        }
    }

    pub fn with_duration(mut self, duration_ms: u128) -> Self {
        self.duration_ms = duration_ms;
        self
    }

    pub fn with_scale_gap(mut self, scale: Vec3) -> Self {
        self.scale_gap = scale;
        self
    }

    pub fn with_ease_function(mut self, ease_function: EaseFunction) -> Self {
        self.function = Some(ease_function);
        self
    }
}

fn easing(
    mut commands: Commands,
    mut query: Query<(&mut Transform, &mut ScaleEasingEffect, Entity)>,
    mut event: EventWriter<ScaleEasingEnded>,
    time: Res<Time>,
) {
    let delta = time.delta().as_millis();
    for (mut transform, mut easing, entity) in query.iter_mut() {
        if easing.elapsed > easing.duration_ms || easing.function.is_none() {
            continue;
        }

        let f = EasingCurve::new(0.0, 1.0, easing.function.unwrap());
        if easing.scale_orig.is_none() {
            easing.scale_orig = Some(transform.scale);
        }
        if easing.scale_start.is_none() {
            easing.scale_start = easing.scale_orig;
        }

        easing.elapsed += delta;
        let percent = easing.elapsed as f32 / easing.duration_ms as f32;
        let rate = f.sample(percent).unwrap_or(1.);
        if let Some(start_scale) = easing.scale_start {
            transform.scale = start_scale + easing.scale_gap * Vec3::splat(rate);
            // transform.translation = start.translation + easing.scale_gap.translation * Vec3::splat(rate);
        }

        if percent >= 1. {
            event.send(ScaleEasingEnded(entity));
            commands.trigger_targets(ScaleEasingEnded(entity), entity);
        }
    }
}
