use bevy::app::App;
use bevy::prelude::{Commands, Component, Entity, Event, Plugin, Query, Res, Timer, Update};
use bevy::time::Time;

pub struct TimerSupportPlugin;

impl Plugin for TimerSupportPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AutoTimerFinished>().add_systems(Update, auto_tick);
    }
}

#[derive(Default)]
pub enum ActionOnFinish {
    #[default]
    Nothing,
    Despawn,
    Remove,
}

#[derive(Component, Default)]
pub struct AutoTimer {
    pub timer: Timer,
    pub action_on_finish: ActionOnFinish,
}

#[derive(Event)]
pub struct AutoTimerFinished;

impl AutoTimer {
    pub fn progress(&self) -> f32 {
        self.timer.elapsed().as_secs_f32() / self.timer.duration().as_secs_f32()
    }
}

fn auto_tick(mut commands: Commands, time: Res<Time>, mut query: Query<(&mut AutoTimer, Entity)>) {
    for (mut timer, e) in query.iter_mut() {
        timer.timer.tick(time.delta());
        if timer.timer.just_finished() {
            commands.trigger_targets(AutoTimerFinished, e);
            match timer.action_on_finish {
                ActionOnFinish::Nothing => {}
                ActionOnFinish::Despawn => {
                    commands.entity(e).despawn();
                }
                ActionOnFinish::Remove => {
                    commands.entity(e).remove::<AutoTimer>();
                }
            }
        }
    }
}
