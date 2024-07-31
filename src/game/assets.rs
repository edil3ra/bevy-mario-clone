use bevy::{
    prelude::*,
    render::texture::{ImageLoaderSettings, ImageSampler},
    utils::HashMap,
};
use bevy_common_assets::json::JsonAssetPlugin;
use serde::Deserialize;

use super::patterns::Pattern;

#[derive(Deserialize, Asset, TypePath, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LevelAsset {
    pub sprite_sheet: String,
    pub pattern_sheet: String,
    pub music_sheet: String,
    pub checkpoints: Vec<[u32; 2]>,
    pub layers: Vec<LevelLayerAsset>,
    pub entities: Vec<LevelEntityAsset>,
    pub triggers: Vec<LevelTriggerAsset>,
}

#[derive(Deserialize, TypePath, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LevelLayerAsset {
    pub tiles: Vec<LevelTileAsset>,
}

#[derive(Deserialize, TypePath, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LevelTileAsset {
    pub style: Option<String>,
    pub pattern: Option<String>,
    pub behavior: Option<String>,
    pub ranges: Vec<Vec<i32>>,
}

#[derive(Deserialize, TypePath, Default, Debug)]
#[serde(rename_all = "camelCase")]
struct LevelEntityAsset {
    name: String,
    pos: [u32; 2],
}

#[derive(Deserialize, TypePath, Default, Debug)]
#[serde(rename_all = "camelCase")]
struct LevelTriggerAsset {
    action: String,
    name: String,
    pos: [u32; 2],
}

#[derive(Deserialize, Asset, TypePath, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SpriteAsset {
    image_url: String,
    tile_w: u8,
    tile_h: u8,
    tiles: Vec<SpriteTileAsset>,
    animations: Vec<AnimationAsset>,
}

#[derive(Deserialize, TypePath, Default, Debug)]
#[serde(rename_all = "camelCase")]
struct SpriteTileAsset {
    name: String,
    index: Option<[u8; 2]>,
}

#[derive(Deserialize, TypePath, Default, Debug)]
#[serde(rename_all = "camelCase")]
struct AnimationAsset {
    name: String,
    frame_len: f32,
    frames: Vec<String>,
}

#[derive(Deserialize, Asset, TypePath, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PatternAsset {
    pub patterns: HashMap<String, PatternTilesAsset>,
}

#[derive(Deserialize, TypePath, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PatternTilesAsset {
    pub tiles: Vec<LevelTileAsset>,
}

// #[derive(Deserialize, TypePath, Default, Debug)]
// #[serde(rename_all = "camelCase")]
// pub struct PatternTileAsset {
//     pub style: Option<String>,
//     pub behaviour: Option<String>,
//     pub ranges: Vec<Vec<i32>>,
// }

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        JsonAssetPlugin::<LevelAsset>::new(&["level.json"]),
        JsonAssetPlugin::<PatternAsset>::new(&["pattern.json"]),
        JsonAssetPlugin::<SpriteAsset>::new(&["sprite.json"]),
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

#[derive(Default, Copy, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum LevelKey {
    #[default]
    OneOne,
    OneTwo,
    OneThree,
    OneFour,
    TwoOne,
}

impl AssetKey for LevelKey {
    type Asset = LevelAsset;
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
    Underwater,
    UnderWorld,
    Castle,
}

impl From<&str> for PatternKey {
    fn from(value: &str) -> Self {
        match value {
            "overworld.pattern" => PatternKey::Overworld,
            "castle.pattern" => PatternKey::Castle,
            "underwater.pattern" => PatternKey::Underwater,
            "underworld.pattern" => PatternKey::Overworld,
            _ => PatternKey::Overworld,
        }
    }
}

impl AssetKey for PatternKey {
    type Asset = PatternAsset;
}

impl FromWorld for HandleMap<PatternKey> {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        [
            (
                PatternKey::Castle,
                asset_server.load("patterns/castle.pattern.json"),
            ),
            (
                PatternKey::Overworld,
                asset_server.load("patterns/overworld.pattern.json"),
            ),
            (
                PatternKey::Underwater,
                asset_server.load("patterns/underwater.pattern.json"),
            ),
            (
                PatternKey::UnderWorld,
                asset_server.load("patterns/underworld.pattern.json"),
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
    type Asset = SpriteAsset;
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
