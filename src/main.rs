use bevy::{platform::collections::HashMap, prelude::*};
use inventory::{Index, Inventory, InventoryPlugin};
use item::{ItemPlugin, Items};

mod inventory;
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

    let sword_a = items.add_item("sword");
    let sword_b = items.add_item("sword");
    let bow = items.add_item("bow");
    let stones_a = items.add_items("stones", 5);
    let stones_b = items.add_items("stones", 10);
    let stones_c = items.add_items("stones", 17);

    let mut i = HashMap::new();
    i.insert(Index(UVec2::new(0, 2)), sword_a);
    i.insert(Index(UVec2::new(1, 2)), sword_b);
    i.insert(Index(UVec2::new(2, 2)), bow);
    i.insert(Index(UVec2::new(3, 0)), stones_a);
    i.insert(Index(UVec2::new(3, 1)), stones_b);
    i.insert(Index(UVec2::new(3, 2)), stones_c);


    commands.spawn((
        Inventory { items: i },
        MyInventoryMarker,
        Visibility::Inherited,
        Node {
            display: Display::Grid,
            align_self: AlignSelf::Center,
            justify_self: JustifySelf::Center,
            width: percent(50),
            height: percent(50),
            ..default()
        },
        //BackgroundColor(Color::WHITE),
    ));
}

//fn draw_slot(
    //mut commands: Commands,
    //inventories: Query<&MyInventoryMarker>,
    //query: Query<(Entity, &Index), Added<Slot>>
//) {
    //query
        //.into_iter()
        ////.filter(|(_, _, child_of)| inventories.get(child_of.parent()).is_ok())
        //.for_each(|(entity, i)| {
            //commands
                //.entity(entity)
                //.with_child((
                    //Text::new(format!("{}:{}", i.x, i.y)),
                    //Node {
                        //width: percent(100),
                        //height: percent(100),
                        //align_self: AlignSelf::Center,
                        //justify_self: JustifySelf::Center,
                        //..default()
                    //},
                    //Pickable::IGNORE,
                    //Node {
                        //width: percent(100),
                        //height: percent(100),
                    //}
                
                //));
        //});
//}

//fn on_save(
    //query: Query<(Entity, Index), (With<Slot>, With<ItemStack<
//) {
    //query
//}

