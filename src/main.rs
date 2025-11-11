use bevy::{platform::collections::HashMap, prelude::*};
use inventory_ui::{GridInventory, Index, InventoryPlugin};
use inventory::Inventories;
use item::{ItemPlugin, Items, ItemType};

mod inventory;
mod inventory_ui;
mod item;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(ItemPlugin)
        .add_plugins(InventoryPlugin)
        .add_systems(Startup, setup)
        .run();
}

#[derive(Component)]
struct MyInventoryMarker;

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
    
    items.register_item_type(ItemType {
        tag: "sword".into(),
        name: "Sword".into(),
        sprite_path: "TODO".into(),
        max_stack_size: 1,
    });

    items.register_item_type(ItemType {
        tag: "bow".into(),
        name: "Bow".into(),
        sprite_path: "TODO".into(),
        max_stack_size: 1,
    });

    items.register_item_type(ItemType {
        tag: "stones".into(),
        name: "Stones".into(),
        sprite_path: "STONES".into(),
        max_stack_size: 20,
    });

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

    let data = inventories.entry_mut("second");
    data.set(Index::new(0, 0), stones_d);
    
    commands.spawn((
        GridInventory::new("main".into()),
        Node {
            align_self: AlignSelf::Center,
            justify_self: JustifySelf::Start,
            width: percent(40),
            height: percent(50),
            ..default()
        },
    ));
    commands.spawn((
        GridInventory::new("second".into()),
        Node {
            align_self: AlignSelf::Center,
            justify_self: JustifySelf::End,
            width: percent(40),
            height: percent(50),
            ..default()
        },
    ));
}

