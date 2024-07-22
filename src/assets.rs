use bevy::{
    prelude::*,
    render::texture::{ImageLoaderSettings, ImageSampler},
    utils::HashMap,
};
use bevy_common_assets::json::JsonAssetPlugin;
use serde::Deserialize;

#[derive(Deserialize, Asset, TypePath, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Level {
    sprite_sheet: String,
    pattern_sheet: String,
    music_sheet: String,
    checkpoints: Vec<[u32; 2]>,
    layers: Vec<LevelLayer>,
    entities: Vec<LevelEntity>,
    triggers: Vec<LevelTrigger>,
}

#[derive(Deserialize, TypePath, Default, Debug)]
#[serde(rename_all = "camelCase")]
struct LevelLayer {
    tiles: Vec<LevelTile>,
}

#[derive(Deserialize, TypePath, Default, Debug)]
#[serde(rename_all = "camelCase")]
struct LevelTile {
    style: Option<String>,
    pattern: Option<String>,
    behavior: Option<String>,
    ranges: Vec<Vec<i32>>,
}

#[derive(Deserialize, TypePath, Default, Debug)]
#[serde(rename_all = "camelCase")]
struct LevelEntity {
    name: String,
    pos: [u32; 2],
}

#[derive(Deserialize, TypePath, Default, Debug)]
#[serde(rename_all = "camelCase")]
struct LevelTrigger {
    action: String,
    name: String,
    pos: [u32; 2],
}

#[derive(Deserialize, Asset, TypePath, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Sprite {
    image_url: String,
    tile_w: u8,
    tile_h: u8,
    tiles: Vec<SpriteTile>,
    animations: Vec<Animation>,
}

#[derive(Deserialize, TypePath, Default, Debug)]
#[serde(rename_all = "camelCase")]
struct SpriteTile {
    name: String,
    index: Option<[u8; 2]>,
}

#[derive(Deserialize, TypePath, Default, Debug)]
#[serde(rename_all = "camelCase")]
struct Animation {
    name: String,
    frame_len: f32,
    frames: Vec<String>,
}

#[derive(Deserialize, Asset, TypePath, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Pattern {
    patterns: HashMap<String, PatternTiles>,
}

#[derive(Deserialize, TypePath, Default, Debug)]
#[serde(rename_all = "camelCase")]
struct PatternTiles {
    tiles: Vec<PatternTile>,
}

#[derive(Deserialize, TypePath, Default, Debug)]
#[serde(rename_all = "camelCase")]
struct PatternTile {
    style: Option<String>,
    behaviour: Option<String>,
    ranges: Vec<Vec<u32>>,
}

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        JsonAssetPlugin::<Level>::new(&["level.json"]),
        JsonAssetPlugin::<Pattern>::new(&["pattern.json"]),
        JsonAssetPlugin::<Sprite>::new(&["sprite.json"]),
    ));
    app.register_type::<HandleMap<TextureKey>>();
    app.init_resource::<HandleMap<TextureKey>>();

    app.register_type::<HandleMap<LevelKey>>();
    app.init_resource::<HandleMap<LevelKey>>();

    app.register_type::<HandleMap<SpriteKey>>();
    app.init_resource::<HandleMap<SpriteKey>>();

    app.register_type::<HandleMap<PatternKey>>();
    app.init_resource::<HandleMap<PatternKey>>();
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum TextureKey {
    Entities,
    Tiles,
}

impl AssetKey for TextureKey {
    type Asset = Image;
}

impl FromWorld for HandleMap<TextureKey> {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        [
            (
                TextureKey::Entities,
                asset_server.load("textures/entities.png"),
            ),
            (TextureKey::Tiles, asset_server.load("textures/tiles.png")),
        ]
        .into()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum LevelKey {
    OneOne,
    OneTwo,
    OneThree,
    OneFour,
    TwoOne,
}

impl AssetKey for LevelKey {
    type Asset = Level;
}

impl FromWorld for HandleMap<LevelKey> {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        [
            (LevelKey::OneOne, asset_server.load("levels/1-1.level.json")),
            (LevelKey::OneTwo, asset_server.load("levels/1-2.level.json")),
            (
                LevelKey::OneThree,
                asset_server.load("levels/1-3.level.json"),
            ),
            (
                LevelKey::OneFour,
                asset_server.load("levels/1-4.level.json"),
            ),
            (LevelKey::TwoOne, asset_server.load("levels/2-1.level.json")),
        ]
        .into()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum PatternKey {
    Overworld,
    Castle,
}

impl AssetKey for PatternKey {
    type Asset = Pattern;
}

impl FromWorld for HandleMap<PatternKey> {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        [
            (
                PatternKey::Overworld,
                asset_server.load("patterns/overworld.pattern.json"),
            ),
            (
                PatternKey::Castle,
                asset_server.load("patterns/castle.pattern.json"),
            ),
        ]
        .into()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum SpriteKey {
    Overworld,
    Mario,
}

impl AssetKey for SpriteKey {
    type Asset = Sprite;
}

impl FromWorld for HandleMap<SpriteKey> {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        [
            (
                SpriteKey::Overworld,
                asset_server.load("sprites/overworld.sprite.json"),
            ),
            (
                SpriteKey::Mario,
                asset_server.load("sprites/castle.sprite.json"),
            ),
        ]
        .into()
    }
}

pub trait AssetKey: Sized {
    type Asset: Asset;
}

#[derive(Resource, Reflect, Deref, DerefMut)]
#[reflect(Resource)]
pub struct HandleMap<K: AssetKey>(HashMap<K, Handle<K::Asset>>);

impl<K: AssetKey, T> From<T> for HandleMap<K>
where
    T: Into<HashMap<K, Handle<K::Asset>>>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl<K: AssetKey> HandleMap<K> {
    pub fn all_loaded(&self, asset_server: &AssetServer) -> bool {
        self.values()
            .all(|x| asset_server.is_loaded_with_dependencies(x))
    }
}
