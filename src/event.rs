use bevy::{ecs::event::EntityComponentsTrigger, prelude::*};


#[derive(Debug, Clone, EntityEvent)]
#[entity_event(trigger = EntityComponentsTrigger<'a>)]
pub struct SlotEvent<E> {
    pub entity: Entity,
    pub event: E,
}

impl<E> SlotEvent<E> {
    pub fn new(entity: Entity, event: E) -> Self {
        SlotEvent {
            entity,
            event,
        }
    }
}

impl<E> core::ops::Deref for SlotEvent<E> {
    type Target = E;

    fn deref(&self) -> &Self::Target {
        &self.event
    }
}

#[derive(Debug)]
pub struct SlotAdd;

#[derive(Debug)]
pub struct SlotOver;

#[derive(Debug)]
pub struct SlotOut;

#[derive(Debug)]
pub struct SlotUpdate;

#[derive(Debug)]
pub struct SlotBackgroundAdd;

#[derive(Debug)]
pub struct SlotBackgroundOver;

#[derive(Debug)]
pub struct SlotBackgroundOut;

#[derive(Debug)]
pub struct SlotDoubleClick;

#[derive(Debug)]
pub struct SlotShiftClick;

pub(crate) trait TriggerSlotEvent {
    fn trigger_slot_event<E: Send + Sync + 'static>(&mut self, event: SlotEvent<E>);
}

impl<'w, 's> TriggerSlotEvent for Commands<'w, 's> {
    fn trigger_slot_event<E: Send + Sync + 'static>(&mut self, event: SlotEvent<E>) {
        self.queue(move |world: &mut World| {
            let Ok(iter) = world.inspect_entity(event.event_target())
            else {
                return
            };

            let ids = iter.map(|c| c.id()).collect::<Vec<_>>();

            world.trigger_with(event, EntityComponentsTrigger {
                components: &ids,
            });
        }); 
    }
}

