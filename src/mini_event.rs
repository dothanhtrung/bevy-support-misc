use std::time::Duration;
use bevy::{
    app::{
        Plugin,
        Update,
    },
    ecs::{
        component::Component,
        entity::Entity,
        event::EntityEvent,
        hierarchy::ChildOf,
        observer::On,
        query::{
            With,
            Without,
        },

        system::{
            Commands,
            Query,

            Single,
        },
    },
    state::state::States,
    time::TimerMode,
};
use bevy_auto_timer::{
    AutoTimer,
    AutoTimerFinished,
    AutoTimerPlugin,
};
use bevy_rand::{
    global::GlobalRng,
    plugin::EntropyPlugin,
    prelude::WyRand,
};
use rand::RngExt;

#[derive(Default)]
pub struct MiniEventSupportPlugin<T>
where
    T: States,
{
    pub states: Vec<T>,
}

impl<T> MiniEventSupportPlugin<T>
where
    T: States,
{
    pub fn new(states: Vec<T>) -> Self {
        Self { states }
    }
}

impl<T> Plugin for MiniEventSupportPlugin<T>
where
    T: States,
{
    fn build(&self, app: &mut bevy::app::App) {
        if !app.is_plugin_added::<EntropyPlugin<WyRand>>() {
            app.add_plugins(EntropyPlugin::<WyRand>::default());
        }
        if !app.is_plugin_added::<AutoTimerPlugin<T>>() {
            app.add_plugins(AutoTimerPlugin::<T>::any());
        }

        app.add_systems(Update, setup)
            .add_observer(start)
            .add_observer(stop)
            .add_observer(event_extend);
    }
}

#[derive(Component, Default)]
struct EventSetup;

#[derive(Component, Default)]
#[require(EventSetup)]
pub struct MiniEvent {
    pub min_wait_ms: u64,
    pub max_wait_ms: u64,
    pub event_time: Duration,
    pub is_repeat: bool,
}

#[derive(EntityEvent)]
pub struct StartMiniEvent {
    pub entity: Entity,
}

#[derive(EntityEvent)]
pub struct StopMiniEvent {
    pub entity: Entity,
}

#[derive(EntityEvent)]
pub struct MiniEventBegin {
    pub entity: Entity,
}

#[derive(EntityEvent)]
pub struct MiniEventEnd {
    pub entity: Entity,
}

#[derive(EntityEvent)]
pub struct MiniEventExtend {
    pub entity: Entity,
    pub duration: Duration,
}

#[derive(Component, Default)]
pub struct WaitTimer;

#[derive(Component, Default)]
pub struct EventTimer;

fn setup(mut commands: Commands, query: Query<(Entity, &MiniEvent), With<EventSetup>>) {
    for (e, mini_event) in query.iter() {
        commands.entity(e).with_children(|parent| {
            let mut wait_timer = AutoTimer::default();
            wait_timer.timer.pause();
            parent.spawn((wait_timer, WaitTimer)).observe(event_begin);

            let mut event_timer = AutoTimer::new(mini_event.event_time, TimerMode::Once);
            event_timer.timer.pause();
            parent.spawn((event_timer, EventTimer)).observe(event_end);
        });

        commands.entity(e).remove::<EventSetup>();
    }
}

fn start(
    trigger: On<StartMiniEvent>,
    mut rng: Single<&mut WyRand, With<GlobalRng>>,
    mini_events: Query<&MiniEvent>,
    mut query: Query<(&mut AutoTimer, &ChildOf), With<WaitTimer>>,
) {
    if let Ok(mini_event) = mini_events.get(trigger.entity) {
        let random_ms = if mini_event.max_wait_ms > mini_event.min_wait_ms {
            rng.random_range(mini_event.min_wait_ms..mini_event.max_wait_ms)
        } else if mini_event.max_wait_ms == mini_event.min_wait_ms {
            mini_event.max_wait_ms
        } else {
            0
        };
        let wait_duration = Duration::from_millis(random_ms);
        for (mut timer, child_of) in query.iter_mut() {
            if child_of.parent() == trigger.entity {
                timer.timer.reset();
                timer.timer.set_duration(wait_duration);
                timer.timer.unpause();
                break;
            }
        }
    }
}

fn stop(
    trigger: On<StopMiniEvent>,
    mut wait_timer_query: Query<(&ChildOf, &mut AutoTimer), (With<WaitTimer>, Without<EventTimer>)>,
    mut event_timer_query: Query<(&ChildOf, &mut AutoTimer), With<EventTimer>>,
) {
    for (child_of, mut wait_timer) in wait_timer_query.iter_mut() {
        if child_of.parent() == trigger.entity {
            wait_timer.timer.pause();
            break;
        }
    }
    for (child_of, mut event_timer) in event_timer_query.iter_mut() {
        if child_of.parent() == trigger.entity {
            event_timer.timer.pause();
            break;
        }
    }
}

fn event_begin(
    trigger: On<AutoTimerFinished>,
    mut commands: Commands,
    mut wait_timer_query: Query<(&ChildOf, &mut AutoTimer), (With<WaitTimer>, Without<EventTimer>)>,
    mut event_timer_query: Query<(&ChildOf, &mut AutoTimer), With<EventTimer>>,
) {
    if let Ok((child_of, mut wait_timer)) = wait_timer_query.get_mut(trigger.entity) {
        let parent_entity = child_of.parent();
        commands.trigger(MiniEventBegin { entity: parent_entity });
        wait_timer.timer.pause();

        for (child_of, mut event_timer) in event_timer_query.iter_mut() {
            if child_of.parent() == parent_entity {
                event_timer.timer.reset();
                event_timer.timer.unpause();
                break;
            }
        }
    }
}

fn event_end(
    trigger: On<AutoTimerFinished>,
    mut commands: Commands,
    mut wait_timer_query: Query<(&ChildOf, &mut AutoTimer), (With<WaitTimer>, Without<EventTimer>)>,
    mut event_timer_query: Query<(&ChildOf, &mut AutoTimer), With<EventTimer>>,
    event_query: Query<&MiniEvent>,
    mut rng: Single<&mut WyRand, With<GlobalRng>>,
) {
    if let Ok((child_of, mut event_timer)) = event_timer_query.get_mut(trigger.entity) {
        let parent_entity = child_of.parent();

        commands.trigger(MiniEventEnd { entity: parent_entity });
        event_timer.timer.pause();

        if let Ok(mini_event) = event_query.get(parent_entity) {
            if !mini_event.is_repeat {
                return;
            }
            for (child_of, mut wait_timer) in wait_timer_query.iter_mut() {
                if child_of.parent() == parent_entity {
                    let random_ms = if mini_event.max_wait_ms > mini_event.min_wait_ms {
                        rng.random_range(mini_event.min_wait_ms..mini_event.max_wait_ms)
                    } else if mini_event.max_wait_ms == mini_event.min_wait_ms {
                        mini_event.max_wait_ms
                    } else {
                        0
                    };
                    wait_timer.timer.reset();
                    wait_timer.timer.set_duration(Duration::from_millis(random_ms));
                    wait_timer.timer.unpause();
                    break;
                }
            }
        }
    }
}

fn event_extend(
    trigger: On<MiniEventExtend>,
    mut event_timer_query: Query<(&ChildOf, &mut AutoTimer), With<EventTimer>>,
) {
    for (child_of, mut event_timer) in event_timer_query.iter_mut() {
        if child_of.parent() == trigger.entity {
            let elapsed = event_timer.timer.elapsed() - trigger.duration;
            event_timer.timer.set_elapsed(elapsed);
            break;
        }
    }
}
