use auto_move::{MovePolicy, on_event_move_to};
use bevy::{asset::ron, prelude::*};
use bevy_pkv::{PersistentResourceAppExtensions, PkvStore};
use plugin::SiminvPlugin;
use simple_renderer::{SiminvSimpleRendererPlugin, SimpleImageHandle, SimpleRendererAssets};
use event::*;
use inventory::{Inventories, Index};
use item::{ItemType, Items, Tag};
use grid::{GridInventoryConfig, build_grid_inventory};
use bevy_asset_loader::prelude::*;

mod auto_move;
mod double_click;
mod shift_click;
mod inventory;
mod slot;
mod slot_background;
mod item;
mod grid;
mod plugin;
mod event;
mod palette;
mod tint;
mod simple_renderer;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum GameState {
    #[default]
    AssetLoading,
    Next,
}

#[derive(AssetCollection, Resource)]
struct GameAssets {
    #[asset(path = "images/slot_bg.png")]
    slot_background: Handle<Image>,
    #[asset(path = "images/slot_bg_over.png")]
    slot_background_over: Handle<Image>,
    #[asset(path = "images/slot_bg_error.png")]
    slot_background_error: Handle<Image>,

    #[asset(texture_atlas_layout(tile_size_x = 32, tile_size_y = 32, columns = 11, rows = 22))]
    icons_atlas: Handle<TextureAtlasLayout>,
    #[asset(image(sampler(filter = nearest)))]
    #[asset(path = "images/icons.png")]
    icons: Handle<Image>,
}

impl GameAssets {
    fn texture_atlast_for_item(&self, item: &str) -> TextureAtlas {
        let index = match item {
            "sword" => 56,
            "shield" => 67,
            "bow" => 69,
            "helmet" => 78,
            "armor" => 84,
            "stones" => 188,
            // empty spot
            _ => {
                // warn here!
                3
            }
        };

        TextureAtlas::from(self.icons_atlas.clone())
            .with_index(index)
    }
}

impl SimpleRendererAssets for GameAssets {
    fn background(&self) -> SimpleImageHandle {
        SimpleImageHandle::Direct(self.slot_background.clone())
    }

    fn background_over(&self) -> SimpleImageHandle {
        SimpleImageHandle::Direct(self.slot_background_over.clone())
    }

    fn background_error(&self) -> SimpleImageHandle {
        SimpleImageHandle::Direct(self.slot_background_error.clone())
    }

    fn item(&self, item: &str) -> SimpleImageHandle {
        SimpleImageHandle::AtlasImage(self.icons.clone(), self.texture_atlast_for_item(item))
    }
}

fn add_to_inventory(
    items: &mut Items,
    inventories: &mut Inventories,
    name: &str, 
    data: impl IntoIterator<Item = ((u32, u32), &'static str)> 
) {
    for (index, item_query) in data {
        let item: Vec<_> = item_query.split(":").collect();
        if item.len() == 1 {
            let item_name = item[0];
            let item_id = items.add_item(item_name);
            inventories.entry_mut(name).set(index.into(), item_id);
        } else if item.len() == 2 {
            let item_name = item[0];
            let count = item[1].parse::<u64>().expect("ok");
            let item_id = items.add_items(item_name, count);
            inventories.entry_mut(name).set(index.into(), item_id);
        }
    }
}

// load defaults here, cause they depend on each other
// TODO: make make them not depend on each other
fn default_resources() -> (Items, Inventories) {
    const ITEMS_RON: &str = include_str!("../assets/data/item_types.ron");
    let item_types: Vec<ItemType> = ron::from_str(ITEMS_RON).expect("Failed to parse item_types.ron");
    let mut items = Items::default();
    items.register_item_types(item_types);

    let mut inventories = Inventories::default();

    let _ = inventories.entry_mut("backpack");
    let _ = inventories.entry_mut("eq");
    let _ = inventories.entry_mut("stash");
    // TODO: initialize with it?
    add_to_inventory(
        &mut items, 
        &mut inventories, 
        "backpack",
        vec![
            ((0, 0), "shield"),
            ((0, 1), "helmet"),
            ((0, 2), "armor"),
            ((0, 3), "sword"),
            ((1, 2), "sword"),
            ((2, 2), "bow"),
            ((3, 0), "stones:5"),
            ((3, 1), "stones:10"),
            ((3, 2), "stones:17"),
        ]
    );

    (items, inventories)
}

fn main() {


    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .init_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading)
                .load_collection::<GameAssets>()
                .continue_to_state(GameState::Next)
        )
        .insert_resource(PkvStore::new("siminv", "example.01"))
        .init_persistent_resource_with(move || {
            println!("setting default items");
            default_resources().0
        })
        .init_persistent_resource_with(move || {
            println!("setting default inventory");
            default_resources().1
        })

        .add_plugins(SiminvPlugin)
        .add_plugins(SiminvSimpleRendererPlugin::<GameAssets, FantasyStyle>::default())
        
        // backpack
        .add_observer(on_event_move_to::<SlotDoubleClick, Backpack, Equipment, { MovePolicy::EMPTY_OR_REPLACE }>)
        .add_observer(on_event_move_to::<SlotShiftClick, Backpack, Stash, { MovePolicy::ONLY_EMPTY }>)

        // equipment
        .add_observer(on_event_move_to::<SlotDoubleClick, Equipment, Backpack, { MovePolicy::ONLY_EMPTY }>)
        .add_observer(on_event_move_to::<SlotShiftClick, Equipment, Stash, { MovePolicy::ONLY_EMPTY }>)
        
        // stash
        .add_observer(on_event_move_to::<SlotDoubleClick, Stash, Equipment, { MovePolicy::EMPTY_OR_REPLACE }>)
        .add_observer(on_event_move_to::<SlotShiftClick, Stash, Backpack, { MovePolicy::ONLY_EMPTY }>)

        .add_systems(OnEnter(GameState::Next), setup)
        .run();
}

#[derive(Component, Default)]
struct FantasyStyle;

#[derive(Component, Default)]
struct Equipment;

#[derive(Component, Default)]
struct Backpack;

#[derive(Component, Default)]
struct Stash;

fn setup(
    mut commands: Commands,
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

    let data = inventories.entry_mut("backpack");
    
    commands.spawn((
        Node {
            width: percent(100),
            height: percent(100),
            ..default()
        },
        BackgroundColor(palette::ColorPair::METAL.light),
        GlobalZIndex(-1),
    ));

    commands.spawn((
        Node {
            align_self: AlignSelf::End,
            justify_self: JustifySelf::Start,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            width: percent(35),
            height: percent(50),
            ..default()
        },
        children![
            build_grid_inventory::<(FantasyStyle, Backpack)>("backpack", data, &GridInventoryConfig {
                slot_width: px(80),
                slot_height: px(80),
                columns: 5, 
                rows: 4,
                row_gap: px(0),
                column_gap: px(0),
                width: px(80 * 5),
                height: px(80 * 4),
                ..default()
            })
        ]
    ));

    let data = inventories.entry_mut("eq");

    // |        | helmet |  neckles |
    // | weapon |armor   | off-hand |
    // | ring   |        | ring     |
    // | gloves | boots  | -        |
    
    commands.spawn((
        Node {
            align_self: AlignSelf::Start,
            justify_self: JustifySelf::Start,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            width: percent(35),
            height: percent(50),
            ..default()
        },
        children![
            build_grid_inventory::<(FantasyStyle, Equipment)>("eq", data, &GridInventoryConfig {
                slot_width: px(80),
                slot_height: px(80),
                columns: 3, 
                rows: 4,
                row_gap: px(0),
                column_gap: px(0),
                width: px(80 * 3),
                height: px(80 * 4),
                required_tags: [
                    (Index::new(1, 0), Tag("helmet".into())),
                    (Index::new(2, 0), Tag("neckles".into())),
                    (Index::new(0, 1), Tag("weapon".into())),
                    (Index::new(1, 1), Tag("armor".into())),
                    (Index::new(2, 1), Tag("off-hand".into())),
                    (Index::new(0, 2), Tag("ring".into())),
                    (Index::new(2, 2), Tag("ring".into())),
                    (Index::new(0, 3), Tag("gloves".into())),
                    (Index::new(1, 3), Tag("boots".into())),
                ].into(),
                blocked_indexes: vec![
                    Index::new(0, 0),
                    Index::new(1, 2),
                    Index::new(2, 3),
                ].into_iter().collect(),
                ..default()
            })
        ]
    ));

    let data = inventories.entry_mut("stash");

    commands.spawn((
        Node {
            align_self: AlignSelf::End,
            justify_self: JustifySelf::End,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            width: percent(35),
            height: percent(100),
            ..default()
        },
        children![
            build_grid_inventory::<(FantasyStyle, Stash)>("stash", data, &GridInventoryConfig {
                slot_width: px(80),
                slot_height: px(80),
                columns: 5, 
                rows: 8,
                row_gap: px(0),
                column_gap: px(0),
                width: px(80 * 5),
                height: px(80 * 8),
                ..default()
            })
        ]
    ));
}

