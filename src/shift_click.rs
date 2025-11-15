use bevy::prelude::*;

use crate::event::{SlotEvent, SlotShiftClick, TriggerSlotEvent};

#[derive(Resource, Default)]
pub(crate) struct ShiftClick {
    shift_pressed: bool,
}

pub(crate) fn detect_shift_press(
    keys: Res<ButtonInput<KeyCode>>,
    mut shift_click: ResMut<ShiftClick>,
) {
    shift_click.shift_pressed = keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight);
}

pub(crate) fn on_click<F: Component>(
    click: On<Pointer<Click>>,
    mut commands: Commands,
    shift_click: Res<ShiftClick>,
    query: Query<&F>
) {
    if !shift_click.shift_pressed {
        return
    }

    // not observed component
    if query.get(click.entity).is_err() {
        return
    }

    commands.trigger_slot_event(SlotEvent::new(click.entity, SlotShiftClick));
}

