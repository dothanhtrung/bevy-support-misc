use crate::easing::DummyState;
use bevy::app::{App, Update};
use bevy::math::Vec3;
use bevy::prelude::{
    default, in_state, Commands, Component, Curve, Deref, DerefMut, EaseFunction, EasingCurve, Entity, Event,
    IntoScheduleConfigs, Plugin, Query, Res, States, Time, Transform,
};

pub struct ScaleEasingPlugin<T>
where
    T: States,
{
    pub states: Option<Vec<T>>,
}

impl<T> ScaleEasingPlugin<T>
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

impl<T> Plugin for ScaleEasingPlugin<T>
where
    T: States,
{
    fn build(&self, app: &mut App) {
        app.add_event::<ScaleEasingEnded>();

        if let Some(states) = &self.states {
            for state in states {
                app.add_systems(Update, easing.run_if(in_state(state.clone())));
            }
        } else {
            app.add_systems(Update, easing);
        }
    }
}

pub struct ScaleEasingPluginAnyState;

impl ScaleEasingPluginAnyState {
    pub fn new() -> ScaleEasingPlugin<DummyState> {
        ScaleEasingPlugin::any()
    }
}

#[derive(Event, Deref, DerefMut)]
pub struct ScaleEasingEnded(pub EaseFunction);

#[derive(Component, Default)]
pub struct ScaleEasingEffect {
    pub scale_gap: Vec3,
    pub scale_orig: Option<Vec3>,
    pub scale_start: Option<Vec3>,
    pub duration_ms: u128,
    elapsed: u128,
    pub function: Option<EaseFunction>,
    pub repeat: bool,
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

    pub fn with_repeat(mut self, repeat: bool) -> Self {
        self.repeat = repeat;
        self
    }

    pub fn reset(&mut self) {
        self.elapsed = 0;
    }
}

fn easing(mut commands: Commands, mut query: Query<(&mut Transform, &mut ScaleEasingEffect, Entity)>, time: Res<Time>) {
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
        }

        if percent >= 1. {
            commands.trigger_targets(ScaleEasingEnded(easing.function.unwrap()), entity);
            if easing.repeat {
                easing.reset();
            }
        }
    }
}
