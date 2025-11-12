use bevy::{asset::ron, prelude::*};
use inventory_ui::{Index, InventoryPlugin, Slot};
use inventory::Inventories;
use item::{ItemPlugin, ItemType, Items, Tag};
use layouts::{build_grid_inventory, GridInventoryConfig};

mod inventory;
mod inventory_ui;
mod item;
mod layouts;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(ItemPlugin)
        .add_plugins(InventoryPlugin)
        .add_systems(Startup, setup)
        .run();
}

#[derive(Component, Default)]
struct MainGrid;

#[derive(Component, Default)]
struct SecondGrid;

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
        Node {
            align_self: AlignSelf::Start,
            justify_self: JustifySelf::End,
            width: percent(50),
            height: percent(50),
            ..default()
        }
    ));
}

