use std::time::Duration;

use bevy::{
    app::{
        Plugin,
        Update,
    },
    ecs::{
        message::{
            Message,
            MessageReader,
            MessageWriter,
        },
        query::With,
        resource::Resource,
        schedule::{
            IntoScheduleConfigs,
            common_conditions::on_message,
        },
        system::{
            Res,
            ResMut,
            Single,
        },
    },
    prelude::{
        Deref,
        DerefMut,
    },
    time::{
        Time,
        Timer,
    },
};
use bevy_rand::{
    global::GlobalRng,
    plugin::EntropyPlugin,
    prelude::WyRand,
};
use rand::RngExt;

pub struct MiniEventSupportPlugin;

impl Plugin for MiniEventSupportPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        if !app.is_plugin_added::<EntropyPlugin<WyRand>>() {
            app.add_plugins(EntropyPlugin::<WyRand>::default());
        }

        app.add_message::<MiniEventBegin>()
            .add_message::<MiniEventEnd>()
            .insert_resource(MiniEventSetting::default())
            .insert_resource(NextMiniEventTimer::default())
            .insert_resource(MiniEventTimer::default())
            .add_systems(
                Update,
                (
                    tick,
                    start.run_if(on_message::<StartMiniEvent>),
                    stop.run_if(on_message::<StopMiniEvent>),
                    event_end.run_if(on_message::<MiniEventEnd>),
                    event_start.run_if(on_message::<MiniEventBegin>),
                    event_extend.run_if(on_message::<MiniEventExtend>),
                ),
            );
    }
}

#[derive(Message)]
pub struct StartMiniEvent;

#[derive(Message)]
pub struct StopMiniEvent;

#[derive(Message)]
pub struct MiniEventBegin;

#[derive(Message)]
pub struct MiniEventEnd;

#[derive(Message, Deref, DerefMut)]
pub struct MiniEventExtend(pub u64);

#[derive(Resource, Default)]
pub struct MiniEventSetting {
    pub min_gap: u64,
    pub max_gap: u64,
}

#[derive(Resource, Default, Deref, DerefMut)]
struct NextMiniEventTimer(Timer);

#[derive(Resource, Default, Deref, DerefMut)]
struct MiniEventTimer(Timer);

fn tick(
    time: Res<Time>,
    mut next_event_timer: ResMut<NextMiniEventTimer>,
    mut mini_event_timer: ResMut<MiniEventTimer>,
    mut event_begin: MessageWriter<MiniEventBegin>,
    mut event_end: MessageWriter<MiniEventEnd>,
) {
    let delta = time.delta();
    if !next_event_timer.is_paused() {
        next_event_timer.tick(delta);
    } else if !mini_event_timer.is_paused() {
        mini_event_timer.tick(delta);
    }

    if next_event_timer.just_finished() {
        event_begin.write(MiniEventBegin);
    } else if mini_event_timer.just_finished() {
        event_end.write(MiniEventEnd);
    }
}

fn start(mut next_timer: ResMut<NextMiniEventTimer>, mut event_timer: ResMut<MiniEventTimer>) {
    next_timer.reset();
    next_timer.unpause();

    event_timer.reset();
    event_timer.pause();
}

fn stop(mut next_timer: ResMut<NextMiniEventTimer>, mut event_timer: ResMut<MiniEventTimer>) {
    next_timer.pause();
    event_timer.pause();
}

fn event_end(
    mut next_event_timer: ResMut<NextMiniEventTimer>,
    mut event_timer: ResMut<MiniEventTimer>,
    mut rng: Single<&mut WyRand, With<GlobalRng>>,
    setting: Res<MiniEventSetting>,
) {
    let next_event_ms = rng.random_range(setting.min_gap..setting.max_gap);
    next_event_timer.set_duration(Duration::from_millis(next_event_ms));
    next_event_timer.reset();
    next_event_timer.unpause();

    event_timer.pause();
}

fn event_start(mut next_event_timer: ResMut<NextMiniEventTimer>, mut event_timer: ResMut<MiniEventTimer>) {
    event_timer.reset();
    event_timer.unpause();

    next_event_timer.pause();
}

fn event_extend(mut msgs: MessageReader<MiniEventExtend>, mut event_timer: ResMut<MiniEventTimer>) {
    for msg in msgs.read() {
        let elapsed = event_timer.elapsed();
        event_timer.set_elapsed(elapsed - Duration::from_millis(**msg));
    }
}
