use crate::timer::AutoTimer;
use bevy::app::{App, Plugin, Update};
use bevy::prelude::{Alpha, Commands, Component, Entity, Query, Res, Sprite, Time, Timer, Visibility};
use bevy::text::TextColor;
use bevy::ui::BackgroundColor;

pub struct FadingSupportPlugin;

impl Plugin for FadingSupportPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, fading);
    }
}

#[derive(Default)]
pub enum Fade {
    #[default]
    In,
    Out,
}

#[derive(Component, Default)]
#[require(Visibility)]
pub struct FadeSupport {
    timer: AutoTimer,
    fade: Fade,
    despawn_on_finish: bool,
}

impl FadeSupport {
    pub fn new(timer: Timer, fade: Fade, despawn_on_finish: bool) -> Self {
        Self {
            timer: AutoTimer(timer),
            fade,
            despawn_on_finish,
        }
    }

    pub fn reset(&mut self) {
        self.timer.reset();
        self.timer.unpause();
    }

    pub fn change(&mut self, fade: Fade) {
        self.fade = fade;
    }
}

fn fading(
    mut commands: Commands,
    mut query: Query<(
        Option<&mut BackgroundColor>,
        Option<&mut TextColor>,
        Option<&mut Sprite>,
        &mut FadeSupport,
        &mut Visibility,
        Entity,
    )>,
    time: Res<Time>,
) {
    for (background_color, text_color, sprite, mut fading, mut visibility, entity) in query.iter_mut() {
        if fading.timer.finished() {
            return;
        }

        fading.timer.tick(time.delta());
        let progress = fading.timer.progress();

        let alpha = match fading.fade {
            Fade::In => {
                *visibility = Visibility::Visible;
                progress
            }
            Fade::Out => 1.0 - progress,
        };
        if let Some(mut bg) = background_color {
            bg.0.set_alpha(alpha);
        }
        if let Some(mut text) = text_color {
            text.0.set_alpha(alpha);
        }

        if let Some(mut spr) = sprite {
            spr.color.set_alpha(alpha);
        }

        if alpha <= 0. {
            *visibility = Visibility::Hidden;

            if fading.despawn_on_finish {
                commands.entity(entity).despawn();
            }
        }
    }
}
