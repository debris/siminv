use bevy::prelude::*;

use crate::event::*;

#[derive(Component)]
pub struct SlotBackground;

pub(crate) fn on_add(
    added: On<Add, SlotBackground> ,
    mut commands: Commands,
) {
    commands.trigger_slot_event(SlotEvent::new(added.entity, SlotBackgroundAdd));
}


pub(crate) fn on_pointer_over(
    over: On<Pointer<Over>>,
    mut commands: Commands,
    query: Query<&SlotBackground>,
) {
    // TODO: handle double trigger from Slot
    if query.get(over.entity).is_ok() {
        commands.trigger_slot_event(SlotEvent::new(over.entity, SlotBackgroundOver));
    }
}

pub(crate) fn on_pointer_out(
    out: On<Pointer<Out>>,
    mut commands: Commands,
    query: Query<&SlotBackground>,
) {
    // TODO: handle double trigger from Slot
    if query.get(out.entity).is_ok() {
        commands.trigger_slot_event(SlotEvent::new(out.entity, SlotBackgroundOut));
    }
}

