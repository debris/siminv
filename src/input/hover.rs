use core::time::Duration;

use bevy::prelude::*;

use crate::event::{SlotEvent, SlotHover, SlotHoverOver, TriggerSlotEvent};

#[derive(Debug, PartialEq)]
enum HoverState {
    None,
    Preparing {
        entity: Entity,
        timer: Timer,
    },
    Triggered,
}

#[derive(Resource)]
pub(crate) struct Hover {
    duration: Duration,
    state: HoverState,
}

impl Default for Hover {
    fn default() -> Self {
        Self {
            duration: Duration::from_secs_f64(0.5),
            state: HoverState::None,
        }
    }
}

pub(crate) fn on_over<F: Component>(
    over: On<Pointer<Over>>,
    query: Query<&F>,
    mut hover: ResMut<Hover>,
){
    if !query.contains(over.entity) {
        return
    }

    if hover.state != HoverState::None {
        // TODO: swap it for logging library
        println!("overlapping hover, it may not work properly");
    }

    hover.state = HoverState::Preparing { 
        entity: over.entity,
        timer: Timer::new(hover.duration, TimerMode::Once),
    };
}

pub(crate) fn on_out<F: Component>(
    out: On<Pointer<Out>>,
    mut commands: Commands,
    query: Query<&F>,
    mut hover: ResMut<Hover>,
) {
    if !query.contains(out.entity) {
        return
    }

    if hover.state == HoverState::Triggered {
        commands.trigger_slot_event(SlotEvent::new(out.entity, SlotHoverOver));
    }

    hover.state = HoverState::None;
}

pub(crate) fn update_time(
    time: ResMut<Time>,
    mut commands: Commands,
    mut hover: ResMut<Hover>,
) {
    match &mut hover.state { 
        HoverState::Preparing { entity, timer } => {
            if timer.is_finished() {
                commands.trigger_slot_event(SlotEvent::new(*entity, SlotHover));
                hover.state = HoverState::Triggered;
            } else {
                timer.tick(time.delta());
            }
        },
        _ => {},
    }
}

