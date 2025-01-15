use bevy::app::App;
use bevy::math::Vec3;
use bevy::prelude::{Changed, Color, Component, Font, Handle, Interaction, Luminance, Plugin, Query, Transform, Update};
use bevy::ui::BackgroundColor;

pub struct GameButtonPlugin;

impl Plugin for GameButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, btn_effect);
    }
}

#[derive(Component)]
pub struct ButtonEffect {
    pub scale: Vec3,
    pub translation: Vec3,
    pub lighter: f32,
    orig_scale: Vec3,
    orig_translation: Vec3,
    orig_color: Color,
    in_effect: bool,
}

impl Default for ButtonEffect {
    fn default() -> Self {
        Self {
            scale: Vec3::new(0.9, 0.9, 1.),
            translation: Vec3::new(0., -5., 0.),
            lighter: 0.1,
            orig_scale: Vec3::new(1., 1., 1.),
            orig_translation: Vec3::default(),
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

pub fn btn_effect(
    mut query: Query<(&mut Transform, &mut ButtonEffect, &mut BackgroundColor, &Interaction), Changed<Interaction>>,
) {
    for (mut transform, mut effect, mut bg_color, interaction) in query.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                effect.in_effect = true;
                transform.scale *= effect.scale;
                transform.translation += effect.translation;
            }
            Interaction::Hovered => {
                effect.in_effect = true;
                transform.scale = effect.orig_scale;
                bg_color.0 = bg_color.0.lighter(effect.lighter);
            }
            Interaction::None => {
                if effect.in_effect {
                    transform.scale = effect.orig_scale;
                    bg_color.0 = effect.orig_color;
                    effect.in_effect = false;
                }
                effect.orig_scale = transform.scale;
                effect.orig_translation = transform.translation;
                effect.orig_color = bg_color.0;
            }
        }
    }
}
