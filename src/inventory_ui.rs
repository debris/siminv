use bevy::prelude::*;

use crate::{inventory::Inventories, item::{ItemId, Items, Tag}};

#[derive(Component, Default)]
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
                BackgroundColor(Color::linear_rgba(0., 0., 0., 0.)),
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
    if let Ok([mut slot_from, mut slot_into]) = query.get_many_mut([on_drag_drop.dropped, on_drag_drop.event_target()]) {
        match (slot_from.item, slot_into.item) {
            // merge or swap them
            (Some(from_id), Some(into_id)) => {
                
                // check if tags are matching
                let from_item = items.get_item_meta(from_id).expect("to be there");
                let into_item = items.get_item_meta(into_id).expect("to be there");
                if !slot_from.matching_tag(&into_item.tags) || !slot_into.matching_tag(&from_item.tags) {
                    return
                }

                
                let (new_from, new_into) = items.merge_or_swap(from_id, into_id).expect("to be no error");
                slot_from.item = new_from;
                slot_into.item = new_into;
            }
            // move slot item onto empty space
            (Some(from_id), None) => {
                let from_item = items.get_item_meta(from_id).expect("to be there");
                if !slot_into.matching_tag(&from_item.tags) {
                    return
                }

                core::mem::swap(&mut slot_from.item, &mut slot_into.item);
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

        let Some(item) = slot.item.and_then(|item_id| items.get_item_meta(item_id))
        else {
            continue
        };

        entity_commands.with_child((
            Text::new(format!("{}: {}", item.display_name, item.stack_size)),
            Pickable::IGNORE,
        ));
    };
}

