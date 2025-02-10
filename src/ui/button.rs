use bevy::app::App;
use bevy::math::Vec3;
use bevy::prelude::{Changed, Color, Component, Font, Handle, Interaction, Luminance, Plugin, Query, Transform, Update};
use bevy::ui::BackgroundColor;

pub struct GameButtonPlugin;

impl Plugin for GameButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (btn_transform_effect, btn_lighter_effect));
    }
}

#[derive(Component)]
pub struct ButtonTransformEffect {
    pub scale: Vec3,
    pub translation: Vec3,
    orig_scale: Vec3,
    orig_translation: Vec3,
    in_effect: bool,
}

#[derive(Component)]
pub struct ButtonLighterEffect {
    pub lighter: f32,
    orig_color: Color,
    in_effect: bool,
}

impl Default for ButtonTransformEffect {
    fn default() -> Self {
        Self {
            scale: Vec3::new(0.9, 0.9, 1.),
            translation: Vec3::new(0., -5., 0.),
            orig_scale: Vec3::new(1., 1., 1.),
            orig_translation: Vec3::default(),
            in_effect: false,
        }
    }
}

impl Default for ButtonLighterEffect {
    fn default() -> Self {
        Self {
            lighter: 0.1,
            orig_color: Color::NONE,
            in_effect: false,
        }
    }
}

pub struct ButtonStyle {
    // Color when pressed
    pub active_bg: Color,
    pub inactive_bg: Color,
    pub bg: Color,
    pub text_color: Color,
    pub font: Handle<Font>,
}

pub fn btn_transform_effect(
    mut query: Query<(&mut Transform, &mut ButtonTransformEffect, &Interaction), Changed<Interaction>>,
) {
    for (mut transform, mut effect, interaction) in query.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                effect.in_effect = true;
                transform.scale = effect.orig_scale * effect.scale;
                transform.translation = effect.orig_translation + effect.translation;
            }
            Interaction::Hovered => {
                effect.in_effect = true;
                transform.scale = effect.orig_scale;
            }
            Interaction::None => {
                if effect.in_effect {
                    transform.scale = effect.orig_scale;
                    effect.in_effect = false;
                }
                effect.orig_scale = transform.scale;
                effect.orig_translation = transform.translation;
            }
        }
    }
}

pub fn btn_lighter_effect(
    mut query: Query<(&mut BackgroundColor, &mut ButtonLighterEffect, &Interaction), Changed<Interaction>>,
) {
    for (mut bg_color, mut effect, interaction) in query.iter_mut() {
        match interaction {
            Interaction::Pressed => {}
            Interaction::Hovered => {
                effect.in_effect = true;
                bg_color.0 = effect.orig_color.lighter(effect.lighter);
            }
            Interaction::None => {
                if effect.in_effect {
                    bg_color.0 = effect.orig_color;
                    effect.in_effect = false;
                }
                effect.orig_color = bg_color.0;
            }
        }
    }
}
