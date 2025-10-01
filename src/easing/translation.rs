use crate::DummyState;
use bevy::app::{App, Update};
use bevy::math::Vec3;
use bevy::prelude::{
    in_state, Commands, Component, Curve, Deref, DerefMut, EaseFunction, EasingCurve, Entity, EntityEvent
    , IntoScheduleConfigs, Plugin, Query, Res, States, Time, Transform,
};

pub struct TranslationEasingPlugin<T>
where
    T: States,
{
    pub states: Option<Vec<T>>,
}

impl<T> TranslationEasingPlugin<T>
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

impl<T> Plugin for TranslationEasingPlugin<T>
where
    T: States,
{
    fn build(&self, app: &mut App) {
        if let Some(states) = &self.states {
            for state in states {
                app.add_systems(Update, easing.run_if(in_state(state.clone())));
            }
        } else {
            app.add_systems(Update, easing);
        }
    }
}

pub struct TranslationEasingPluginAnyState;

impl TranslationEasingPluginAnyState {
    pub fn new() -> TranslationEasingPlugin<DummyState> {
        TranslationEasingPlugin::any()
    }
}

#[derive(EntityEvent, Deref, DerefMut)]
pub struct TranslationEasingEnded(pub Entity);

#[derive(Component, Default)]
pub struct TranslationEasingEffect {
    translation_gap: Vec3,
    translation_orig: Option<Vec3>,
    translation_start: Option<Vec3>,
    duration_ms: u128,
    elapsed: u128,
    function: Option<EaseFunction>,
}

impl TranslationEasingEffect {
    pub fn orig_translation(&self) -> Option<Vec3> {
        self.translation_orig
    }

    pub fn shake(translation_gap: Vec3, translation_orig: Vec3, duration_ms: u128) -> Self {
        Self {
            translation_gap,
            translation_orig: Some(translation_orig),
            translation_start: Some(translation_orig - translation_gap),
            duration_ms,
            elapsed: 0,
            function: Some(EaseFunction::Elastic(50.)),
        }
    }

    pub fn with_duration(mut self, duration_ms: u128) -> Self {
        self.duration_ms = duration_ms;
        self
    }

    pub fn with_translation_gap(mut self, translation: Vec3) -> Self {
        self.translation_gap = translation;
        self
    }

    pub fn with_ease_function(mut self, ease_function: EaseFunction) -> Self {
        self.function = Some(ease_function);
        self
    }
}

fn easing(
    mut commands: Commands,
    mut query: Query<(&mut Transform, &mut TranslationEasingEffect, Entity)>,
    time: Res<Time>,
) {
    let delta = time.delta().as_millis();
    for (mut transform, mut easing, entity) in query.iter_mut() {
        if easing.elapsed > easing.duration_ms || easing.function.is_none() {
            continue;
        }

        let f = EasingCurve::new(0.0, 1.0, easing.function.unwrap());
        if easing.translation_orig.is_none() {
            easing.translation_orig = Some(transform.translation);
        }
        if easing.translation_start.is_none() {
            easing.translation_start = easing.translation_orig;
        }

        easing.elapsed += delta;
        let percent = easing.elapsed as f32 / easing.duration_ms as f32;
        let rate = f.sample(percent).unwrap_or(1.);
        if let Some(start_translation) = easing.translation_start {
            transform.translation = start_translation + easing.translation_gap * Vec3::splat(rate);
        }

        if percent >= 1. {
            commands.trigger(TranslationEasingEnded(entity));
        }
    }
}
