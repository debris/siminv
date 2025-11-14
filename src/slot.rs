use bevy::prelude::*;

use crate::{event::*, item::{ItemId, Items, Tag}};

#[derive(Component, Default, Debug)]
pub struct Slot {
    pub item: Option<ItemId>,
    pub required_tag: Option<Tag>,
}

impl Slot {
    pub fn empty() -> Self {
        Slot::default()
    }

    pub fn with_item(item: ItemId) -> Self {
        Slot {
            item: Some(item),
            required_tag: None,
        }
    }

    pub fn with_required_tag(tag: Tag) -> Self {
        Slot {
            item: None,
            required_tag: Some(tag),
        }
    }

    pub fn matching_tag(&self, tags: &[Tag]) -> bool {
        match self.required_tag {
            None => true,
            Some(ref tag) => tags.contains(tag)
        }
    }
}

pub(crate) fn on_add(
    added: On<Add, Slot>,
    mut commands: Commands,
) {
    commands.entity(added.entity)
        .try_insert((
            Pickable {
                should_block_lower: false,
                is_hoverable: true,
            },
            GlobalZIndex(0i32),
        ));

    commands.trigger_slot_event(SlotEvent::new(added.entity, SlotAdd));
}

pub(crate) fn on_pointer_over(
    mut over: On<Pointer<Over>>,
    mut commands: Commands,
    query: Query<&Slot>,
) {
    if query.get(over.entity).is_ok() {
        // do not propagate to slot background
        // it has its own observer
        over.propagate(false);
        commands.trigger_slot_event(SlotEvent::new(over.entity, SlotOver));
    }
}

pub(crate) fn on_pointer_out(
    mut out: On<Pointer<Out>>,
    mut commands: Commands,
    query: Query<&Slot>,
) {
    if query.get(out.entity).is_ok() {
        // do not propagate to slot background
        // it has its own observer
        out.propagate(false);
        commands.trigger_slot_event(SlotEvent::new(out.entity, SlotOut));
    }
}

pub(crate) fn on_pointer_drag_start(
    on_drag_start: On<Pointer<DragStart>>,
    mut query: Query<(&Slot, &mut GlobalZIndex), With<Slot>>,
) {
    if let Ok((slot, mut z_index)) = query.get_mut(on_drag_start.event_target()) {
        // we can only drag items that have something inside
        if slot.item.is_some() {
            // we are draggin it. it should always be on the top
            z_index.0 = 1000;
        }
    }
}

pub(crate) fn on_pointer_drag(
    on_drag: On<Pointer<Drag>>,
    mut query: Query<(&Slot, &mut UiTransform), With<Slot>>,
) {
    if let Ok((slot, mut transform)) = query.get_mut(on_drag.event_target()) {
        // we can only drag items that have something inside
        if slot.item.is_some() {
            transform.translation = Val2::px(on_drag.distance.x, on_drag.distance.y);
        }
    }
}

pub(crate) fn on_pointer_drag_end(
    on_drag_end: On<Pointer<DragEnd>>,
    mut query: Query<(&mut UiTransform, &mut GlobalZIndex), With<Slot>>,
) {
    if let Ok((mut transform, mut z_index)) = query.get_mut(on_drag_end.event_target()) {
        transform.translation = Val2::ZERO;
        z_index.0 = 0;
    }
}

pub(crate) fn on_pointer_drag_drop(
    on_drag_drop: On<Pointer<DragDrop>>,
    mut commands: Commands,
    mut query: Query<&mut Slot>,
    mut items: ResMut<Items>,
) {
    if let Ok([mut slot_from, mut slot_into]) = query.get_many_mut([on_drag_drop.dropped, on_drag_drop.event_target()]) {
        match (slot_from.item, slot_into.item) {
            // merge or swap them
            (Some(from_id), Some(into_id)) => {
                
                // check if tags are matching
                let from_item = items.get_item_meta(from_id).expect("to be there");
                let into_item = items.get_item_meta(into_id).expect("to be there");
                if !slot_from.matching_tag(into_item.tags) || !slot_into.matching_tag(from_item.tags) {
                    return
                }

                
                let (new_from, new_into) = items.merge_or_swap(from_id, into_id).expect("to be no error");
                slot_from.item = new_from;
                slot_into.item = new_into;

                // unselect the previous item to prevent blink before pointer out triggers
                commands.trigger_slot_event(SlotEvent::new(on_drag_drop.dropped, SlotOut));
            }
            // move slot item onto empty space
            (Some(from_id), None) => {
                let from_item = items.get_item_meta(from_id).expect("to be there");
                if !slot_into.matching_tag(from_item.tags) {
                    return
                }

                core::mem::swap(&mut slot_from.item, &mut slot_into.item);

                // unselect the previous item to prevent blink before pointer out triggers
                commands.trigger_slot_event(SlotEvent::new(on_drag_drop.dropped, SlotOut));
            }
            // nothing if the grabbed slot does not contain an item
            _ => {
            }
        }

        // TODO: update inventory here
    }
}

pub(crate) fn update_slot(
    mut commands: Commands,
    query: Query<(Entity, &Slot), Changed<Slot>>
) {
    // should we move it to drag drop?
    for (entity, slot) in query {
        commands.trigger_slot_event(SlotEvent::new(entity, SlotUpdate {
            item: slot.item,
            required_tag: slot.required_tag.clone(),
        }));
    };
}

