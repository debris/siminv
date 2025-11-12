use bevy::{asset::ron, prelude::*};
use slot::{Slot, SlotAdd, SlotEvent, SlotOut, SlotOver, SlotPlugin, SlotUpdate};
use inventory::{Inventories, Index};
use item::{ItemPlugin, ItemType, Items, Tag};
use layouts::{build_grid_inventory, GridInventoryConfig};

mod inventory;
mod slot;
mod item;
mod layouts;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(ItemPlugin)
        .add_plugins(SlotPlugin)
        .add_systems(Startup, setup)
        .add_observer(on_slot_add)
        .add_observer(on_slot_over)
        .add_observer(on_slot_out)
        .add_observer(on_main_grid_slot_update)
        .add_observer(on_big_weapon_slot_update)
        .add_observer(on_second_grid_update)
        .run();
}

#[derive(Component, Default)]
struct MainGrid;

#[derive(Component, Default)]
struct SecondGrid;

#[derive(Component)]
struct BigWeaponSlot;

fn setup(
    mut commands: Commands,
    mut items: ResMut<Items>,
    mut inventories: ResMut<Inventories>,
) {

    let projection = OrthographicProjection {
        scaling_mode: bevy::camera::ScalingMode::AutoMin { min_width: 800., min_height: 600. },
        ..OrthographicProjection::default_2d()
    };


    commands.spawn((
        Name::new("camera"),
        Camera2d,
        Projection::Orthographic(projection)
    ));
    
    const ITEMS_RON: &str = include_str!("../assets/data/item_types.ron");
    let item_types: Vec<ItemType> = ron::from_str(ITEMS_RON).expect("Failed to parse item_types.ron");

    for item_type in item_types {
        println!("item type: {:?}", item_type);
        items.register_item_type(item_type);
    }

    let sword_a = items.add_item("sword");
    let sword_b = items.add_item("sword");
    let bow = items.add_item("bow");
    let stones_a = items.add_items("stones", 5);
    let stones_b = items.add_items("stones", 10);
    let stones_c = items.add_items("stones", 17);
    let stones_d = items.add_items("stones", 5);

    let data = inventories.entry_mut("main");
    data.set(Index::new(0, 2), sword_a);
    data.set(Index::new(1, 2), sword_b);
    data.set(Index::new(2, 2), bow);
    data.set(Index::new(3, 0), stones_a);
    data.set(Index::new(3, 1), stones_b);
    data.set(Index::new(3, 2), stones_c);

    
    commands.spawn((
        Node {
            align_self: AlignSelf::Start,
            justify_self: JustifySelf::Start,
            width: percent(50),
            height: percent(50),
            ..default()
        },
        children![
            build_grid_inventory::<MainGrid>(data, &GridInventoryConfig::default())
        ]
    ));

    let data2 = inventories.entry_mut("second");
    data2.set(Index::new(0, 0), stones_d);

    commands.spawn((
        Node {
            align_self: AlignSelf::End,
            justify_self: JustifySelf::End,
            width: percent(50),
            height: percent(50),
            ..default()
        },
        children![
            build_grid_inventory::<SecondGrid>(data2, &GridInventoryConfig {
                slot_width: px(80),
                slot_height: px(80),
                columns: 2, 
                rows: 2,
                ..default()
            })
        ]
    ));

    commands.spawn((
        Slot::with_required_tag(Tag("weapon".into())),
        BigWeaponSlot,
        Node {
            align_self: AlignSelf::Start,
            justify_self: JustifySelf::End,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            width: percent(50),
            height: percent(50),
            ..default()
        }
    ));
}

static MAIN_COLOR: Color = Color::linear_rgba(0., 0., 0., 0.);
static OVER_COLOR: Color = Color::linear_rgba(0.25, 0.25, 0.25, 0.25);

fn on_slot_add(
    add: On<SlotEvent<SlotAdd>, Slot>,
    mut commands: Commands,
) {
    commands.entity(add.entity)
        .try_insert(BackgroundColor(MAIN_COLOR));
}

fn on_slot_over(
    over: On<SlotEvent<SlotOver>, Slot>,
    mut commands: Commands,
) {
    commands.entity(over.entity)
        .try_insert(BackgroundColor(OVER_COLOR));
}

fn on_slot_out(
    out: On<SlotEvent<SlotOut>, Slot>,
    mut commands: Commands,
) {
    commands.entity(out.entity)
        .try_insert(BackgroundColor(MAIN_COLOR));
}

fn on_main_grid_slot_update(
    update: On<SlotEvent<SlotUpdate>, MainGrid>,
    mut commands: Commands,
    items: Res<Items>,
) {
    let display_text = match update.item {
        Some(item_id) => items.get_item_meta(item_id)
            .map(|item| format!("{} {}", item.display_name, item.stack_size)).expect("to be there"),
        None => "".to_owned(),
    };

    commands.entity(update.entity)
        .despawn_children()
        .with_child((
            Text::new(display_text),
            Pickable::IGNORE,
        ));
}

fn on_big_weapon_slot_update(
    update: On<SlotEvent<SlotUpdate>, BigWeaponSlot>,
    mut commands: Commands,
    items: Res<Items>,
) {
    let display_text = match update.item {
        Some(item_id) => items.get_item_meta(item_id)
            .map(|item| item.display_name).expect("to be there"),
        None => "[weapon slot]",
    };

    commands.entity(update.entity)
        .despawn_children()
        .with_child((
            Text::new(display_text),
            Pickable::IGNORE,
        ));
}

fn on_second_grid_update(
    update: On<SlotEvent<SlotUpdate>, SecondGrid>,
    mut commands: Commands,
) {
    let display_text = match update.item {
        Some(_) => "?",
        None => "x",
    };
    
    commands.entity(update.entity)
        .despawn_children()
        .with_child((
            Text::new(display_text),
            Pickable::IGNORE,
        ));
}

