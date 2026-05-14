use std::time::Duration;

use bevy::prelude::*;
use bevy_auto_timer::{
    AutoTimer,
    AutoTimerFinished,
};
use bevy_support_misc::{
    DummyState,
    mini_event::{
        MiniEvent,
        MiniEventBegin,
        MiniEventEnd,
        MiniEventSupportPlugin,
        StartMiniEvent,
    },
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MiniEventSupportPlugin::<DummyState>::new(Vec::new()))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands
        .spawn(MiniEvent {
            min_wait_ms: 500,
            max_wait_ms: 1000,
            is_repeat: true,
            event_time: Duration::from_secs(1),
        })
        .observe(event_begin)
        .observe(event_end);

    commands
        .spawn(AutoTimer::new(Duration::from_millis(500), TimerMode::Once))
        .observe(start_event);
}

fn start_event(_: On<AutoTimerFinished>, query: Query<Entity, With<MiniEvent>>, mut commands: Commands) {
    for entity in query.iter() {
        commands.trigger(StartMiniEvent { entity });
    }
}

fn event_begin(_: On<MiniEventBegin>) {
    info!("Mini Event Begin");
}

fn event_end(_: On<MiniEventEnd>) {
    info!("Mini Event End");
}
