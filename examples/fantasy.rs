use bevy::{asset::ron, prelude::*};
use bevy_pkv::{PersistentResourceAppExtensions, PkvStore};
use siminv::prelude::*;
use siminv::simple_renderer::{SiminvSimpleRendererPlugin, SimpleImageHandle, SimpleRendererAssets};
use bevy_asset_loader::prelude::*;

const BACKGROUND_COLOR: Color = Color::srgb(0.533, 0.584, 0.624);
const RESOLUTION: UVec2 = UVec2::new(1280, 720);

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

// load defaults here, cause they depend on each other
// TODO: make make them not depend on each other
fn default_resources() -> (Items, Inventory) {
    const ITEMS_RON: &str = include_str!("../assets/data/item_types.ron");
    let item_types: Vec<ItemType> = ron::from_str(ITEMS_RON).expect("Failed to parse item_types.ron");
    let mut items = Items::default();
    items.register_item_types(item_types);

    let mut inventory = Inventory::default();
    inventory.set_max_size("stash".into(), UVec2::new(5, 8));
    inventory.set_max_size("backpack".into(), UVec2::new(5, 4));
    inventory.set_max_size("equipment".into(), UVec2::new(3, 4));

    inventory.add("backpack", items.add_item("shield"));
    inventory.add("backpack", items.add_item("sword"));
    inventory.add("backpack", items.add_item("sword"));
    inventory.add("backpack", items.add_item("helmet"));
    inventory.add("backpack", items.add_item("bow"));
    inventory.add("backpack", items.add_item("armor"));
    inventory.add("backpack", items.add_items("stones", 5));
    inventory.add("backpack", items.add_items("stones", 10));
    inventory.add("backpack", items.add_items("stones", 17));

    (items, inventory)
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Siminv: Fantasy Inventory Example".into(),
                    resolution: RESOLUTION.into(),
                    ..default()
                }),
                ..default()
            })
        )
        .init_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading)
                .load_collection::<GameAssets>()
                .continue_to_state(GameState::Next)
        )
        .insert_resource(PkvStore::new("siminv", "example.06"))
        .init_persistent_resource_with(move || {
            println!("setting default items");
            default_resources().0
        })
        .init_persistent_resource_with(move || {
            println!("setting default inventory");
            default_resources().1
        })

        .insert_resource(UiScale(1.0))
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
		.add_systems(Update, update_ui_scale)
        .add_observer(on_button_press)
        .run();
}

/// Marker that is used by the renderer plugin.
#[derive(Component, Default)]
struct FantasyStyle;

/// Marker for equipment. Used to track click events.
#[derive(Component, Default)]
struct Equipment;

/// Marker for backpack. Used to track click events.
#[derive(Component, Default)]
struct Backpack;

/// Marker for stash. Used to track click events.
#[derive(Component, Default)]
struct Stash;

fn update_ui_scale(
    window: Single<&Window>,
    mut ui_scale: ResMut<UiScale>,
) {
    let scale = (window.width() / RESOLUTION.x as f32).min(window.height() / RESOLUTION.y as f32);
    if scale != ui_scale.0 {
        ui_scale.0 = scale;
    }
}

fn setup(
    mut commands: Commands,
) {

    let projection = OrthographicProjection {
        scaling_mode: bevy::camera::ScalingMode::AutoMin { min_width: RESOLUTION.x as f32, min_height: RESOLUTION.y as f32 },
        ..OrthographicProjection::default_2d()
    };


    commands.spawn((
        Name::new("camera"),
        Camera2d,
        Projection::Orthographic(projection)
    ));

    commands.spawn((
        Node {
            width: percent(100),
            height: percent(100),
            ..default()
        },
        BackgroundColor(BACKGROUND_COLOR),
        GlobalZIndex(-1),
    ));

    let grid_style = GridStyle {
        slot_width: px(80),
        slot_height: px(80),
        ..default()
    };

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
            build_grid_inventory::<(FantasyStyle, Backpack)>(&grid_style, &GridInventoryConfig {
                collection: "backpack",
                columns: 5, 
                rows: 4,
                ..default()
            })
        ]
    ));

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
            build_grid_inventory::<(FantasyStyle, Equipment)>(&grid_style, &GridInventoryConfig {
                collection: "equipment",
                columns: 3, 
                rows: 4,
                required_tags: [
                    (UVec2::new(1, 0), Tag("helmet".into())),
                    (UVec2::new(2, 0), Tag("neckles".into())),
                    (UVec2::new(0, 1), Tag("weapon".into())),
                    (UVec2::new(1, 1), Tag("armor".into())),
                    (UVec2::new(2, 1), Tag("off-hand".into())),
                    (UVec2::new(0, 2), Tag("ring".into())),
                    (UVec2::new(2, 2), Tag("ring".into())),
                    (UVec2::new(0, 3), Tag("gloves".into())),
                    (UVec2::new(1, 3), Tag("boots".into())),
                ].into(),
                blocked_indexes: vec![
                    UVec2::new(0, 0),
                    UVec2::new(1, 2),
                    UVec2::new(2, 3),
                ].into_iter().collect(),
                ..default()
            })
        ]
    ));

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
            build_grid_inventory::<(FantasyStyle, Stash)>(&grid_style, &GridInventoryConfig {
                collection: "stash",
                columns: 5, 
                rows: 8,
                ..default()
            })
        ]
    ));

    commands.spawn((
        Node {
            align_self: AlignSelf::Center,
            justify_self: JustifySelf::Center,
            width: px(120),
            height: px(80),
            ..default()
        },
        Text::new("Get Sword"),
        AddButton,
    ));
}

#[derive(Component)]
struct AddButton;

fn on_button_press(
    clicked: On<Pointer<Click>>,
    query: Query<&AddButton>,
    mut items: ResMut<Items>,
    mut inventory: ResMut<Inventory>,
) {
    if !query.contains(clicked.entity) {
        return
    }

    inventory.add("stash",items.add_item("sword"));
}

