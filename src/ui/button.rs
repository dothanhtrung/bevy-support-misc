use bevy::app::App;
use bevy::prelude::{px, Changed, Color, Component, ImageNode, Interaction, IntoScheduleConfigs, Luminance, Plugin, Query, Text, UiTransform, Update, Val2, Vec2};
use bevy::ui::BackgroundColor;

pub struct GameButtonPlugin;

impl Plugin for GameButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (btn_transform_effect, btn_toggle_effect, btn_color_effect).chain(),
        );
    }
}

#[derive(Component)]
#[require(Interaction, UiTransform)]
pub struct ButtonTransformEffect {
    pub scale: Vec2,
    pub translation: Val2,
    orig_scale: Vec2,
    orig_translation: Val2,
    in_effect: bool,
}

impl Default for ButtonTransformEffect {
    fn default() -> Self {
        Self {
            scale: Vec2::new(0.9, 0.9),
            translation: Val2::new(px(0.), px(-5.), ),
            orig_scale: Vec2::new(1., 1.),
            orig_translation: Val2::default(),
            in_effect: false,
        }
    }
}

#[derive(Component)]
#[require(Interaction, BackgroundColor)]
pub struct ButtonColorEffect {
    pub lighter: f32,
    orig_bg_color: Color,
    orig_img_color: Color,
    in_effect: bool,
}

impl Default for ButtonColorEffect {
    fn default() -> Self {
        Self {
            lighter: 0.1,
            orig_bg_color: Color::NONE,
            orig_img_color: Color::NONE,
            in_effect: false,
        }
    }
}
impl ButtonColorEffect {
    pub fn new(lighter: f32) -> Self {
        Self {
            lighter,
            ..Self::default()
        }
    }
}

#[derive(Component)]
#[require(BackgroundColor, Text, Interaction)]
pub struct ButtonToggleEffect {
    pub on_text: String,
    pub off_text: String,
    pub on_color: Color,
    pub off_color: Color,
    pub enabled: bool,
}

impl Default for ButtonToggleEffect {
    fn default() -> Self {
        Self {
            on_text: String::from("On"),
            off_text: String::from("Off"),
            on_color: Color::NONE,
            off_color: Color::NONE,
            enabled: false,
        }
    }
}

fn btn_transform_effect(
    mut query: Query<(&mut UiTransform, &mut ButtonTransformEffect, &Interaction), Changed<Interaction>>,
) {
    for (mut transform, mut effect, interaction) in query.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                effect.in_effect = true;
                transform.scale = effect.orig_scale * effect.scale;
                // TODO: Make translation change
                // transform.translation = effect.orig_translation + effect.translation;
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

fn btn_color_effect(
    mut query: Query<
        (
            &mut BackgroundColor,
            Option<&mut ImageNode>,
            &mut ButtonColorEffect,
            &Interaction,
        ),
        Changed<Interaction>,
    >,
) {
    for (mut bg_color, image_node, mut effect, interaction) in query.iter_mut() {
        match interaction {
            Interaction::Pressed => {}
            Interaction::Hovered => {
                effect.in_effect = true;
                bg_color.0 = effect.orig_bg_color.lighter(effect.lighter);
                if let Some(mut node) = image_node {
                    node.color = node.color.lighter(effect.lighter);
                }
            }
            Interaction::None => {
                if effect.in_effect {
                    bg_color.0 = effect.orig_bg_color;
                    effect.in_effect = false;
                }
                effect.orig_bg_color = bg_color.0;

                if let Some(node) = &image_node {
                    effect.orig_img_color = node.color;
                }
                if effect.in_effect {
                    if let Some(mut node) = image_node {
                        node.color = effect.orig_img_color;
                    }
                }
            }
        }
    }
}

fn btn_toggle_effect(
    mut query: Query<
        (
            &mut ButtonToggleEffect,
            Option<&mut ButtonColorEffect>,
            &mut Text,
            &mut BackgroundColor,
            &Interaction,
        ),
        Changed<Interaction>,
    >,
) {
    for (mut btn_toggle_effect, btn_color_effect_opt, mut text, mut bg_color, interaction) in query.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                btn_toggle_effect.enabled = !btn_toggle_effect.enabled;
                (**text, *bg_color) = if btn_toggle_effect.enabled {
                    (btn_toggle_effect.on_text.clone(), btn_toggle_effect.on_color.into())
                } else {
                    (btn_toggle_effect.off_text.clone(), btn_toggle_effect.off_color.into())
                };

                if let Some(mut btn_color_effect) = btn_color_effect_opt {
                    btn_color_effect.orig_bg_color = bg_color.0;
                }
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}
