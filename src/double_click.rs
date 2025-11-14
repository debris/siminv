use std::time::Duration;

use bevy::prelude::*;

use crate::{event::{SlotDoubleClick, SlotEvent, TriggerSlotEvent}, slot::Slot};

#[derive(Resource)]
pub(crate) struct DoubleClick {
    duration: Duration,
    timer: Option<Timer>
}

impl Default for DoubleClick {
    fn default() -> Self {
        DoubleClick {
            duration: Duration::from_secs_f64(0.2),
            timer: None,
        }
    }
}

pub(crate) fn on_click(
    click: On<Pointer<Click>>,
    mut commands: Commands,
    mut double_click: ResMut<DoubleClick>,
    query: Query<&Slot>,
) {
    // not slot
    if !query.get(click.entity).is_ok() {
        return
    }

    match double_click.timer {
        None => {
            double_click.timer = Some(Timer::new(double_click.duration, TimerMode::Once));
        },
        Some(_) => {
            println!("here on_click");
            double_click.timer = None;
            // the click may be on different entities, but since it is supposed to
            // be fast, we don't care
            commands.trigger_slot_event(SlotEvent::new(click.entity, SlotDoubleClick));
        }
    }
}

pub(crate) fn update_time(
    time: ResMut<Time>,
    mut click: ResMut<DoubleClick>,
) {
    if let Some(timer) = &mut click.timer {
        if timer.is_finished() {
            click.timer = None;
        } else {
            timer.tick(time.delta());
        }
    }
}

