use bevy::app::App;
use bevy::prelude::{Commands, Component, Deref, DerefMut, Entity, Event, Plugin, Query, Res, Timer, Update};
use bevy::time::Time;

pub struct TimerSupportPlugin;

impl Plugin for TimerSupportPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AutoTimerFinished>().add_systems(Update, auto_tick);
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct AutoTimer(pub Timer);

#[derive(Event)]
pub struct AutoTimerFinished;

impl AutoTimer {
    pub fn progress(&self) -> f32 {
        self.0.elapsed().as_secs_f32() / self.0.duration().as_secs_f32()
    }
}

fn auto_tick(mut commands: Commands, time: Res<Time>, mut query: Query<(&mut AutoTimer, Entity)>) {
    for (mut timer, e) in query.iter_mut() {
        if !timer.paused() {
            timer.tick(time.delta());
            if timer.just_finished() {
                commands.trigger_targets(AutoTimerFinished, e);
            }
        }
    }
}
