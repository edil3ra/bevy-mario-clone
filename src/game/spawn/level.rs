use bevy::prelude::*;

use super::{map::SpawnMap, player::SpawnPlayer};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_level);
}

#[derive(Event, Debug)]
pub struct SpawnLevel;

pub enum OverWorld {
    Ground = 0,
    Sky = 126,
    Chocolate = 3,
    Bricks = 1,
    BricksTop = 14,
    Metal = 2,
    Chance1 = 4,
    Chance2 = 5,
    Chance3 = 6,
    Coin1 = 15,
    Coin2 = 31,
    Coin3 = 47,
    PipeInsertVertLeft = 80,
    PipeInsertVertRight = 81,
    PipeVertLeft = 96,
    PipeVertRight = 97,
    PipeChromeInsertVertLeft = 50,
    PipeChromeInsertVertRight = 51,
    PipeChromeVertLeft = 66,
    PipeChromeVertRight = 67,
    PipeInsertHorTop = 54,
    PipeInsertHorBottom = 70,
    PipeHorTop = 55,
    PipeHorBottom = 71,
    PipeConnHorTop = 56,
    PipeConnHorBottom = 72,
    CloudTile = 142,
    Cloud11 = 171,
    Cloud12 = 172,
    Cloud13 = 173,
    Cloud21 = 187,
    Cloud22 = 188,
    Cloud23 = 189,
    Cannon1 = 62,
    Cannon2 = 78,
    Cannon3 = 94,
    Bush1 = 203,
    Bush2 = 204,
    Bush3 = 205,
    GrassLeft = 41,
    Grass = 42,
    GrassRight = 43,
    Dirt = 13,
    TileBlack = 125,
    TileLightBlue = 122,
    CastleTopClosed = 8,
    CastleTopOpen = 9,
    CastleWindowRight = 10,
    CastleArch = 11,
    CastleWindowLeft = 12,
    PoleGreen = 220,
    PoleWhite = 219,
    PoleFinialDarkGrey = 135,
    PoleFinialGreen = 136,
    HillLeft = 116,
    HillRight = 118,
    HillTop = 119,
    HillStainsRight = 117,
    HillStainsLeft = 115,
    TileGreen = 124,
    TreeLargeTop = 90,
    TreeLargeBottom = 106,
    TreeSmall = 92,
    TreeWhiteLargeTop = 91,
    TreeWhiteLargeBottom = 107,
    TreeWhiteSmall = 93,
    TreeTrunk = 108,
    Fence = 110,
    Bridge = 109,
    BridgeRailGreen = 190,
    BridgeRailWhite = 174,
    Waves = 112,
}
impl From<&str> for OverWorld {
    fn from(value: &str) -> Self {
        match value {
            "ground" => Self::Ground,
            "sky" => Self::Sky,
            "chocolate" => Self::Chocolate,
            "bricks" => Self::Bricks,
            "bricks-top" => Self::BricksTop,
            "metal" => Self::Metal,
            "chance-1" => Self::Chance1,
            "chance-2" => Self::Chance2,
            "chance-3" => Self::Chance3,
            "coin-1" => Self::Coin1,
            "coin-2" => Self::Coin2,
            "coin-3" => Self::Coin3,
            "pipe-insert-vert-left" => Self::PipeInsertVertLeft,
            "pipe-insert-vert-right" => Self::PipeInsertVertRight,
            "pipe-vert-left" => Self::PipeVertLeft,
            "pipe-vert-right" => Self::PipeVertRight,
            "pipe-chrome-insert-vert-left" => Self::PipeChromeInsertVertLeft,
            "pipe-chrome-insert-vert-right" => Self::PipeChromeInsertVertRight,
            "pipe-chrome-vert-left" => Self::PipeChromeVertLeft,
            "pipe-chrome-vert-right" => Self::PipeChromeVertRight,
            "pipe-insert-hor-top" => Self::PipeInsertHorTop,
            "pipe-insert-hor-bottom" => Self::PipeInsertHorBottom,
            "pipe-hor-top" => Self::PipeHorTop,
            "pipe-hor-bottom" => Self::PipeHorBottom,
            "pipe-conn-hor-top" => Self::PipeConnHorTop,
            "pipe-conn-hor-bottom" => Self::PipeConnHorBottom,
            "cloud-tile" => Self::CloudTile,
            "cloud-1-1" => Self::Cloud11,
            "cloud-1-2" => Self::Cloud12,
            "cloud-1-3" => Self::Cloud13,
            "cloud-2-1" => Self::Cloud21,
            "cloud-2-2" => Self::Cloud22,
            "cloud-2-3" => Self::Cloud23,
            "cannon-1" => Self::Cannon1,
            "cannon-2" => Self::Cannon2,
            "cannon-3" => Self::Cannon3,
            "bush-1" => Self::Bush1,
            "bush-2" => Self::Bush2,
            "bush-3" => Self::Bush3,
            "grass-left" => Self::GrassLeft,
            "grass" => Self::Grass,
            "grass-right" => Self::GrassRight,
            "dirt" => Self::Dirt,
            "tile-black" => Self::TileBlack,
            "tile-light-blue" => Self::TileLightBlue,
            "castle-top-closed" => Self::CastleTopClosed,
            "castle-top-open" => Self::CastleTopOpen,
            "castle-window-right" => Self::CastleWindowRight,
            "castle-arch" => Self::CastleArch,
            "castle-window-left" => Self::CastleWindowLeft,
            "pole-green" => Self::PoleGreen,
            "pole-white" => Self::PoleWhite,
            "pole-finial-dark-grey" => Self::PoleFinialDarkGrey,
            "pole-finial-green" => Self::PoleFinialGreen,
            "hill-left" => Self::HillLeft,
            "hill-right" => Self::HillRight,
            "hill-top" => Self::HillTop,
            "hill-stains-right" => Self::HillStainsRight,
            "hill-stains-left" => Self::HillStainsLeft,
            "tile-green" => Self::TileGreen,
            "tree-large-top" => Self::TreeLargeTop,
            "tree-large-bottom" => Self::TreeLargeBottom,
            "tree-small" => Self::TreeSmall,
            "tree-white-large-top" => Self::TreeWhiteLargeTop,
            "tree-white-large-bottom" => Self::TreeWhiteLargeBottom,
            "tree-white-small" => Self::TreeWhiteSmall,
            "tree-trunk" => Self::TreeTrunk,
            "fence" => Self::Fence,
            "bridge" => Self::Bridge,
            "bridge-rail-green" => Self::BridgeRailGreen,
            "bridge-rail-white" => Self::BridgeRailWhite,
            "waves" => Self::Waves,
            _ => Self::Waves,
        }
    }
}

fn spawn_level(_trigger: Trigger<SpawnLevel>, mut commands: Commands) {
    commands.trigger(SpawnPlayer);
    commands.trigger(SpawnMap);
}
