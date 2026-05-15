use std::time::Duration;

use bevy::{
    app::{
        Plugin,
        Update,
    },
    ecs::{
        component::Component, message::{
            Message,
            MessageReader,
            MessageWriter,
        }, observer::On, query::With, resource::Resource, schedule::{
            IntoScheduleConfigs,
            common_conditions::on_message,
        }, system::{
            Commands,
            Res,
            ResMut,
            Single,
        }
    },
    prelude::{
        Deref,
        DerefMut,
    },
    time::{
        Time,
        Timer,
        TimerMode,
    },
};
use bevy_auto_timer::{
    AutoTimer,
    AutoTimerFinished,
    AutoTimerPlugin,
    AutoTimerPluginAnyState,
    DummyState,
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
        // TODO: Match the gamestate
        if !app.is_plugin_added::<AutoTimerPlugin<DummyState>>() {
            app.add_plugins(AutoTimerPluginAnyState::any());
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

#[derive(Component)]
struct MiniEvent(u64);

#[derive(Message, Deref, DerefMut)]
pub struct StartMiniEvent(pub u64);

#[derive(Message, Deref, DerefMut)]
pub struct StopMiniEvent(pub u64);

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

fn start(
    mut commands: Commands,
    mut messages: MessageReader<StartMiniEvent>,
    setting: Res<MiniEventSetting>,
    mut rng: Single<&mut WyRand, With<GlobalRng>>,
) {
    for msg in messages.read() {
        let next_event_ms = if setting.max_gap > setting.min_gap {
            rng.random_range(setting.min_gap..setting.max_gap)
        } else if setting.max_gap == setting.min_gap {
            setting.max_gap
        } else {
            0
        };
        commands
            .spawn((
                AutoTimer::from_seconds(Duration::from_millis(next_event_ms).as_secs_f32(), TimerMode::Once),
                MiniEvent(**msg),
            ))
            .observe(event_start);
    }
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
    let next_event_ms = if setting.max_gap > setting.min_gap {
        rng.random_range(setting.min_gap..setting.max_gap)
    } else if setting.max_gap == setting.min_gap {
        setting.max_gap
    } else {
        0
    };

    next_event_timer.set_duration(Duration::from_millis(next_event_ms));
    next_event_timer.reset();
    next_event_timer.unpause();

    event_timer.pause();
}

fn event_start(
    _: On<AutoTimerFinished>,
    mut next_event_timer: ResMut<NextMiniEventTimer>,
    mut event_timer: ResMut<MiniEventTimer>,
) {
    event_timer.reset();
    event_timer.unpause();
}

fn event_extend(mut msgs: MessageReader<MiniEventExtend>, mut event_timer: ResMut<MiniEventTimer>) {
    for msg in msgs.read() {
        let elapsed = event_timer.elapsed();
        event_timer.set_elapsed(elapsed - Duration::from_millis(**msg));
    }
}
