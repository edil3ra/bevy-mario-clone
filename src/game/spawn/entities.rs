use core::fmt;

use bevy::{prelude::*, utils::hashbrown::HashMap};

use crate::{
    config::{FRAMES_RECT_GOOMBA_BROWN, FRAMES_RECT_PLAYER, MAP_HEIGHT, TILE_SIZE},
    game::{
        assets::{HandleMap, LevelAsset, LevelKey, TextureKey},
        entities::{spawn_goomba, spawn_player},
        GameState,
    },
    screen::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Loading), setup_entities);
}

#[derive(Resource)]
pub struct TextureAtlasLayoutEntities(pub HashMap<EntityKey, Handle<TextureAtlasLayout>>);

#[derive(Event, Debug)]
pub struct SpawnEntities;

#[derive(Component, Debug, Clone, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Mob {}

#[derive(Hash, Default, Eq, PartialEq, Clone, Copy, Reflect)]
pub enum EntityKey {
    #[default]
    Player,
    GoombaBrown,
}

impl fmt::Display for EntityKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EntityKey::Player => write!(f, "Player"),
            EntityKey::GoombaBrown => write!(f, "GoombaBrown"),
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
    pub fn new(key: EntityKey, x: u32, y: u32) -> Self {
        Self { key, x, y }
    }
}

pub fn setup_entities(
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

pub fn spawn_entities(
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

    let enemies = level
        .entities
        .iter()
        .map(|entity_asset| {
            SpawnEntityTemplate::new(
                EntityKey::from(entity_asset.name.as_ref()),
                entity_asset.pos[0],
                entity_asset.pos[1],
            )
        })
        .map(|template| match template.key {
            EntityKey::GoombaBrown => spawn_goomba(
                &mut commands,
                &image_handles,
                &atlas_layout_handles,
                template.key,
                template.x,
                (MAP_HEIGHT * TILE_SIZE) - template.y,
            ),
            _ => commands.spawn_empty().id(),
        })
        .collect::<Vec<_>>();
    commands
        .spawn_empty()
        .insert((
            SpatialBundle {
                ..Default::default()
            },
            Name::new("Enemies"),
        ))
        .push_children(&enemies);
}
