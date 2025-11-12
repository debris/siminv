use bevy::{ecs::entity, prelude::*};

use crate::{inventory::Inventories, item::{ItemId, Items}};

const SLOTS: UVec2 = UVec2::new(4, 4);
const SIZE: Vec2 = Vec2::new(64., 64.);
const MOVE_SPEED: f32 = 1000.;

#[derive(Component)]
pub struct GridInventory {
    tag: String,
}

impl GridInventory {
    pub fn new(tag: String) -> Self {
        Self {
            tag
        }
    }
}

#[derive(Component, Default)]
pub struct Slot {
    pub item: Option<ItemId>,
}

#[derive(Component)]
pub struct SlotColor(Color);

#[derive(Component, Deref, DerefMut, Hash, PartialEq, Clone, Copy, Eq, Debug)]
pub struct Index(pub UVec2);

impl Index {
    pub fn new(x: u32, y: u32) -> Self {
        Self(UVec2::new(x, y))
    }
}

#[derive(Component, Deref, DerefMut, Clone, Copy, PartialEq)]
pub struct InventoryHandle(pub Entity);

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        
        app
            .init_resource::<Inventories>()
            .add_systems(Update, spawn_grid_inventory)
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

fn spawn_grid_inventory(
    mut commands: Commands,
    inventories: Res<Inventories>,
    query: Query<(Entity, &GridInventory), Added<GridInventory>>,
) {
    query
        .into_iter()
        .for_each(move |(entity, inv)| {
            commands.entity(entity)
                .with_child((
                    Node {
                        display: Display::Grid,
                        align_self: AlignSelf::Center,
                        justify_self: JustifySelf::Center,
                        width: percent(100),
                        height: percent(100),
                        ..default()
                    },
                    Children::spawn((0..SLOTS.x)
                        .flat_map(|x| (0..SLOTS.y).map(move |y| (x, y)))
                        .map(|(x, y)| { 
                            let index = Index(UVec2::new(x, y));
                            (
                                Node {
                                    position_type: PositionType::Absolute,
                                    grid_column: GridPlacement::start(x as i16 + 1),
                                    grid_row: GridPlacement::start(y as i16 + 1),
                                    align_items: AlignItems::Center,
                                    justify_items: JustifyItems::Center,
                                    width: percent(100),
                                    height: percent(100),
                                    ..default()
                                },
                                Slot {
                                    item: inventories.data(&inv.tag).expect("TODO").get(&index).copied()
                                },
                                index,
                            )
                        }).collect::<Vec<_>>())
                ));
        });
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
                SlotColor(Color::BLACK),
                BackgroundColor(Color::BLACK),
                InventoryHandle(entity),
                GlobalZIndex(0i32),
            ));
    }
}

fn on_pointer_over(
    over: On<Pointer<Over>>,
    mut query: Query<(&SlotColor, &mut BackgroundColor), With<Slot>>,
) {
    if let Ok((slot_color, mut background_color)) = query.get_mut(over.entity) {
        background_color.0 = slot_color.0.lighter(0.1);
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
    mut query: Query<&mut GlobalZIndex, With<Slot>>,
) {
    if let Ok(mut z_index) = query.get_mut(on_drag_start.event_target()) {
        // we are draggin it. it should always be on the top
        println!("set to 1k");
        z_index.0 = 1000;
    }
}

fn on_pointer_drag(
    on_drag: On<Pointer<Drag>>,
    mut query: Query<&mut UiTransform, With<Slot>>,
) {
    if let Ok(mut transform) = query.get_mut(on_drag.event_target()) {
        transform.translation = Val2::px(on_drag.distance.x, on_drag.distance.y);
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
        if let Some(item_id) = slot.item && let Some(into_id) = into.item {
            let (new_item, new_into) = items.merge_or_swap(item_id, into_id).expect("to be no error");
            slot.item = new_item;
            into.item = new_into;
        } else {
            core::mem::swap(&mut slot.item, &mut into.item);
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

