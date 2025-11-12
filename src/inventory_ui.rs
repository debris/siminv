use bevy::prelude::*;

use crate::{inventory::Inventories, item::{ItemId, Items}};

#[derive(Component, Default)]
pub struct Slot {
    pub item: Option<ItemId>,
}

#[derive(Component)]
pub struct SlotColor(Color);

#[derive(Component)]
pub struct OverColor(Color);

#[derive(Component, Deref, DerefMut, Hash, PartialEq, Clone, Copy, Eq, Debug)]
pub struct Index(pub UVec2);

impl Index {
    pub fn new(x: u32, y: u32) -> Self {
        Self(UVec2::new(x, y))
    }
}

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        
        app
            .init_resource::<Inventories>()
            .add_systems(Update, setup_slot)
            .add_observer(on_pointer_over)
            .add_observer(on_pointer_out)
            .add_observer(on_pointer_drag_start)
            .add_observer(on_pointer_drag)
            .add_observer(on_pointer_drag_end)
            .add_observer(on_pointer_drag_drop)
            .add_systems(Update, update_slot);
    }
}

fn setup_slot(
    mut commands: Commands,
    query: Query<Entity, Added<Slot>>,
) {
    for entity in query {
        commands.entity(entity)
            .try_insert((
                Pickable {
                    should_block_lower: false,
                    is_hoverable: true,
                },
                SlotColor(Color::linear_rgba(0., 0., 0., 0.)),
                OverColor(Color::linear_rgba(0.25, 0.25, 0.25, 0.25),),
                BackgroundColor(Color::BLACK),
                GlobalZIndex(0i32),
            ));
    }
}

fn on_pointer_over(
    over: On<Pointer<Over>>,
    mut query: Query<(&OverColor, &mut BackgroundColor), With<Slot>>,
) {
    if let Ok((over_color, mut background_color)) = query.get_mut(over.entity) {
        background_color.0 = over_color.0;
    }
}

fn on_pointer_out(
    out: On<Pointer<Out>>,
    mut query: Query<(&SlotColor, &mut BackgroundColor), With<Slot>>,
) {
    if let Ok((slot_color, mut background_color)) = query.get_mut(out.entity) {
        background_color.0 = slot_color.0;
    }
}

fn on_pointer_drag_start(
    on_drag_start: On<Pointer<DragStart>>,
    mut query: Query<(&Slot, &mut GlobalZIndex), With<Slot>>,
) {
    if let Ok((slot, mut z_index)) = query.get_mut(on_drag_start.event_target()) {
        // we can only drag items that have something inside
        if slot.item.is_some() {
            // we are draggin it. it should always be on the top
            println!("set to 1k");
            z_index.0 = 1000;
        }
    }
}

fn on_pointer_drag(
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

fn on_pointer_drag_end(
    on_drag_end: On<Pointer<DragEnd>>,
    mut query: Query<(&mut UiTransform, &mut GlobalZIndex), With<Slot>>,
) {
    if let Ok((mut transform, mut z_index)) = query.get_mut(on_drag_end.event_target()) {
        transform.translation = Val2::ZERO;

        println!("set to 0");
        z_index.0 = 0;
    }
}

fn on_pointer_drag_drop(
    on_drag_drop: On<Pointer<DragDrop>>,
    mut query: Query<&mut Slot>,
    mut items: ResMut<Items>,
) {
    if let Ok([mut into, mut slot]) = query.get_many_mut([on_drag_drop.event_target(), on_drag_drop.dropped]) {
        match (slot.item, into.item) {
            // merge or swap them
            (Some(item_id), Some(into_id)) => {
                let (new_item, new_into) = items.merge_or_swap(item_id, into_id).expect("to be no error");
                slot.item = new_item;
                into.item = new_into;
            }
            // move slot item onto empty space
            (Some(_item_id), None) => {
                core::mem::swap(&mut slot.item, &mut into.item);
            }
            // nothing if the grabbed slot does not contain an item
            _ => {
            }
        }

        // TODO: update inventory here
    }
}

fn update_slot(
    mut commands: Commands,
    items: Res<Items>,
    query: Query<(Entity, &Slot), Changed<Slot>>
) {
    
    for (entity, slot) in query {
        let mut entity_commands = commands.entity(entity);
        entity_commands.despawn_children();

        let Some((item, item_type)) = slot.item.and_then(|item_id| items.get_item_meta(item_id))
        else {
            continue
        };

        entity_commands.with_child((
            Text::new(format!("{}: {}", item_type.name, item.stack_size)),
            Pickable::IGNORE,
        ));
    };
}

