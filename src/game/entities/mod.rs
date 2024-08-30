pub mod goomba;
pub mod player;

use bevy::{prelude::*, utils::hashbrown::HashMap};

use crate::{
    config::{MAP_HEIGHT, TILE_SIZE},
    game::assets::{HandleMap, TextureKey},
    screen::Screen, AppSet,
};

use self::{
    goomba::{spawn_goomba, FRAMES_RECT_GOOMBA_BROWN},
    player::{jump, spawn_player, FRAMES_RECT_PLAYER},
};

use super::{
    assets::{LevelAsset, LevelKey},
    GameState,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Loading), setup_entities);
    app.add_systems(Update, (jump).in_set(AppSet::Update));
    app.observe(spawn_entities);
}

#[derive(Resource)]
pub struct TextureAtlasLayoutEntities(HashMap<EntityKey, Handle<TextureAtlasLayout>>);

#[derive(Event, Debug)]
pub struct SpawnEntities;

#[derive(Component, Debug, Clone, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Player {}

#[derive(Component, Debug, Clone, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Mob {}

#[derive(Hash, Default, Eq, PartialEq, Clone, Copy, Reflect)]
pub enum EntityKey {
    #[default]
    Player,
    GoombaBrown,
}

impl EntityKey {
    fn to_string(&self) -> &str {
        match *self {
            EntityKey::GoombaBrown => "GoombaBrown",
            EntityKey::Player => "Player",
        }
    }
}

impl From<&str> for EntityKey {
    fn from(value: &str) -> Self {
        match value {
            "goomba-brown" => Self::GoombaBrown,
            _ => Self::GoombaBrown,
        }
    }
}

pub struct SpawnEntityTemplate {
    key: EntityKey,
    x: u32,
    y: u32,
}

impl SpawnEntityTemplate {
    fn new(key: EntityKey, x: u32, y: u32) -> Self {
        Self { key, x, y }
    }
}

fn setup_entities(
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let layout_props = [
        (EntityKey::Player, FRAMES_RECT_PLAYER.as_ref()),
        (EntityKey::GoombaBrown, FRAMES_RECT_GOOMBA_BROWN.as_ref()),
    ];

    let mut atlas_layouts: HashMap<EntityKey, Handle<TextureAtlasLayout>> = HashMap::new();

    for (key, frames) in layout_props {
        let mut layout = TextureAtlasLayout::new_empty(UVec2::new(29 * 8, 29 * 8));
        for frame_rect in frames {
            layout.add_texture(URect::new(
                frame_rect[0],
                frame_rect[1],
                frame_rect[0] + frame_rect[2],
                frame_rect[1] + frame_rect[3],
            ));
        }
        let handle = texture_atlas_layouts.add(layout);
        atlas_layouts.insert(key, handle);
    }
    commands.insert_resource(TextureAtlasLayoutEntities(atlas_layouts));
}

fn spawn_entities(
    _trigger: Trigger<SpawnEntities>,
    mut commands: Commands,
    image_handles: Res<HandleMap<TextureKey>>,
    level_handles: Res<HandleMap<LevelKey>>,
    atlas_layout_handles: ResMut<TextureAtlasLayoutEntities>,
    levels: ResMut<Assets<LevelAsset>>,
    gs: Res<GameState>,
) {
    let level = levels
        .get(level_handles[&gs.current_level].clone_weak().id())
        .unwrap();

    spawn_player(
        &mut commands,
        &image_handles,
        &atlas_layout_handles,
        EntityKey::Player,
        50,
        30,
    );

    for entity_template in level.entities.iter().map(|entity| {
        SpawnEntityTemplate::new(
            EntityKey::from(entity.name.as_ref()),
            entity.pos[0],
            entity.pos[1],
        )
    }) {
        let SpawnEntityTemplate { key, x, y } = entity_template;
        match key {
            EntityKey::GoombaBrown => spawn_goomba(
                &mut commands,
                &image_handles,
                &atlas_layout_handles,
                key,
                x,
                (MAP_HEIGHT * TILE_SIZE) - y,
            ),
            _ => {}
        }
    }
}
