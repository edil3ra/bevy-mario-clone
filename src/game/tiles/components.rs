use bevy::prelude::*;

use std::time::Duration;

#[derive(Debug, Clone)]
pub struct TileCollision {
    pub from: Entity,
    pub to: Entity,
    pub x_side: Option<XSide>,
    pub y_side: Option<YSide>,
}
use crate::AppSet;

#[derive(Debug, Default, Clone, Copy)]
pub enum XSide {
    #[default]
    Left,
    Right,
}

#[derive(Debug, Default, Clone, Copy)]
pub enum YSide {
    #[default]
    Top,
    Bottom,
}

#[derive(Default, Clone, Debug)]
pub struct Tile {
    pub name: TileName,
    pub animation: AnimationTileBuilder,
    pub behaviour: Behaviour,
}

impl Tile {
    pub fn to_single(name: TileName, frame: u32, behaviour: Behaviour) -> Self {
        Tile {
            name,
            animation: AnimationTileBuilder::Single(frame),
            behaviour,
        }
    }

    pub fn to_multiple(
        name: TileName,
        frames: Vec<u32>,
        frame_duration: Duration,
        behaviour: Behaviour,
    ) -> Self {
        Tile {
            name,
            animation: AnimationTileBuilder::Multiple {
                frames,
                frame_duration,
            },
            behaviour,
        }
    }
}

#[derive(Component, Reflect, Default, Clone, Copy, Debug)]
#[reflect(Component)]
#[derive(strum_macros::Display)]
pub enum TileName {
    #[default]
    Ground,
    Sky,
    Chocolate,
    Bricks,
    BricksTop,
    Metal,
    Chance,
    Chance1,
    Chance2,
    Chance3,
    Coin,
    Coin1,
    Coin2,
    Coin3,
    PipeInsertVertLeft,
    PipeInsertVertRight,
    PipeVertLeft,
    PipeVertRight,
    PipeChromeInsertVertLeft,
    PipeChromeInsertVertRight,
    PipeChromeVertLeft,
    PipeChromeVertRight,
    PipeInsertHorTop,
    PipeInsertHorBottom,
    PipeHorTop,
    PipeHorBottom,
    PipeConnHorTop,
    PipeConnHorBottom,
    Cloud,
    Cloud11,
    Cloud12,
    Cloud13,
    Cloud21,
    Cloud22,
    Cloud23,
    Cannon1,
    Cannon2,
    Cannon3,
    Bush1,
    Bush2,
    Bush3,
    GrassLeft,
    Grass,
    GrassRight,
    Dirt,
    Black,
    LightBLue,
    CastleTopClosed,
    CastleTopOpen,
    CastleWindowRight,
    CastleArch,
    CastleWindowLeft,
    PoleGreen,
    PoleWhite,
    PoleFinialDarkGrey,
    PoleFinialGreen,
    HillLeft,
    HillRight,
    HillTop,
    HillStainsRight,
    HillStainsLeft,
    Green,
    TreeLargeTop,
    TreeLargeBottom,
    TreeSmall,
    TreeWhiteLargeTop,
    TreeWhiteLargeBottom,
    TreeWhiteSmall,
    TreeTrunk,
    Fence,
    Bridge,
    BridgeRailGreen,
    BridgeRailWhite,
    Waves,
}

#[derive(Clone, Debug)]
pub enum AnimationTileBuilder {
    Single(u32),
    Multiple {
        frames: Vec<u32>,
        frame_duration: Duration,
    },
}
impl Default for AnimationTileBuilder {
    fn default() -> Self {
        Self::Single(0)
    }
}

#[derive(Component, Reflect, Clone, Debug)]
#[reflect(Component)]
pub struct AnimationTile {
    pub frames: Vec<u32>,
    pub frame: usize,
    pub timer: Timer,
}

#[derive(Component, Reflect, Default, Clone, Debug)]
#[reflect(Component)]
pub enum Behaviour {
    #[default]
    None,
    Ground,
    Brick,
    Coin,
}

impl Behaviour {
    pub fn is_solid(&self) -> bool {
        match self {
            Behaviour::None => false,
            Behaviour::Ground => true,
            Behaviour::Brick => true,
            Behaviour::Coin => false,
        }
    }
}

impl From<&str> for Tile {
    fn from(value: &str) -> Self {
        match value {
            "ground" => Tile::to_single(TileName::Ground, 0, Behaviour::Ground),
            "sky" => Tile::to_single(TileName::Sky, 126, Behaviour::None),
            "chocolate" => Tile::to_single(TileName::Chocolate, 3, Behaviour::None),
            "bricks" => Tile::to_single(TileName::Bricks, 1, Behaviour::Brick),
            "bricks-top" => Tile::to_single(TileName::BricksTop, 14, Behaviour::Brick),
            "metal" => Tile::to_single(TileName::Metal, 2, Behaviour::Brick),
            "chance" => Tile::to_multiple(
                TileName::Chance,
                [4, 5, 6].into(),
                Duration::from_millis(200),
                Behaviour::None,
            ),
            "coin" => Tile::to_multiple(
                TileName::Chance,
                [15, 31, 47].into(),
                Duration::from_millis(200),
                Behaviour::None,
            ),
            "pipe-insert-vert-left" => {
                Tile::to_single(TileName::PipeInsertVertLeft, 80, Behaviour::Ground)
            }
            "pipe-insert-vert-right" => {
                Tile::to_single(TileName::PipeInsertVertRight, 81, Behaviour::Ground)
            }
            "pipe-vert-left" => Tile::to_single(TileName::PipeVertLeft, 96, Behaviour::Ground),
            "pipe-vert-right" => Tile::to_single(TileName::PipeVertRight, 97, Behaviour::Ground),
            "pipe-chrome-insert-vert-left" => {
                Tile::to_single(TileName::PipeChromeInsertVertLeft, 50, Behaviour::Ground)
            }
            "pipe-chrome-insert-vert-right" => {
                Tile::to_single(TileName::PipeChromeInsertVertRight, 51, Behaviour::Ground)
            }
            "pipe-chrome-vert-left" => {
                Tile::to_single(TileName::PipeChromeVertLeft, 66, Behaviour::Ground)
            }
            "pipe-chrome-vert-right" => {
                Tile::to_single(TileName::PipeChromeVertRight, 67, Behaviour::Ground)
            }
            "pipe-insert-hor-top" => {
                Tile::to_single(TileName::PipeInsertHorTop, 54, Behaviour::Ground)
            }
            "pipe-insert-hor-bottom" => {
                Tile::to_single(TileName::PipeInsertHorBottom, 70, Behaviour::Ground)
            }
            "pipe-hor-top" => Tile::to_single(TileName::PipeHorTop, 54, Behaviour::Ground),
            "pipe-hor-bottom" => Tile::to_single(TileName::PipeHorBottom, 70, Behaviour::Ground),
            "pipe-conn-hor-top" => Tile::to_single(TileName::PipeConnHorTop, 56, Behaviour::Ground),
            "pipe-conn-hor-bottom" => {
                Tile::to_single(TileName::PipeConnHorBottom, 71, Behaviour::Ground)
            }
            "cloud-tile" => Tile::to_single(TileName::Cloud, 142, Behaviour::None),
            "cloud-1-1" => Tile::to_single(TileName::Cloud11, 171, Behaviour::None),
            "cloud-1-2" => Tile::to_single(TileName::Cloud12, 172, Behaviour::None),
            "cloud-1-3" => Tile::to_single(TileName::Cloud13, 173, Behaviour::None),
            "cloud-2-1" => Tile::to_single(TileName::Cloud21, 187, Behaviour::None),
            "cloud-2-2" => Tile::to_single(TileName::Cloud22, 188, Behaviour::None),
            "cloud-2-3" => Tile::to_single(TileName::Cloud23, 189, Behaviour::None),
            "cannon-1" => Tile::to_single(TileName::Cannon1, 62, Behaviour::None),
            "cannon-2" => Tile::to_single(TileName::Cannon2, 78, Behaviour::None),
            "cannon-3" => Tile::to_single(TileName::Cannon3, 94, Behaviour::None),
            "bush-1" => Tile::to_single(TileName::Bush1, 203, Behaviour::None),
            "bush-2" => Tile::to_single(TileName::Bush2, 204, Behaviour::None),
            "bush-3" => Tile::to_single(TileName::Bush3, 205, Behaviour::None),
            "grass-left" => Tile::to_single(TileName::GrassLeft, 41, Behaviour::None),
            "grass" => Tile::to_single(TileName::Grass, 42, Behaviour::None),
            "grass-right" => Tile::to_single(TileName::GrassRight, 43, Behaviour::None),
            "dirt" => Tile::to_single(TileName::Dirt, 13, Behaviour::None),
            "tile-black" => Tile::to_single(TileName::Black, 125, Behaviour::None),
            "tile-light-blue" => Tile::to_single(TileName::LightBLue, 122, Behaviour::None),
            "castle-top-closed" => Tile::to_single(TileName::CastleTopClosed, 8, Behaviour::None),
            "castle-top-open" => Tile::to_single(TileName::CastleTopOpen, 9, Behaviour::None),
            "castle-window-right" => {
                Tile::to_single(TileName::CastleWindowRight, 10, Behaviour::None)
            }
            "castle-arch" => Tile::to_single(TileName::CastleArch, 11, Behaviour::None),
            "castle-window-left" => {
                Tile::to_single(TileName::CastleWindowLeft, 12, Behaviour::None)
            }
            "pole-green" => Tile::to_single(TileName::PoleGreen, 220, Behaviour::None),
            "pole-white" => Tile::to_single(TileName::PoleWhite, 219, Behaviour::None),
            "pole-finial-dark-grey" => {
                Tile::to_single(TileName::PoleFinialDarkGrey, 135, Behaviour::None)
            }
            "pole-finial-green" => Tile::to_single(TileName::PoleFinialGreen, 136, Behaviour::None),
            "hill-left" => Tile::to_single(TileName::HillLeft, 116, Behaviour::None),
            "hill-right" => Tile::to_single(TileName::HillRight, 118, Behaviour::None),
            "hill-top" => Tile::to_single(TileName::HillTop, 119, Behaviour::None),
            "hill-stains-right" => Tile::to_single(TileName::HillStainsRight, 117, Behaviour::None),
            "hill-stains-left" => Tile::to_single(TileName::HillStainsLeft, 115, Behaviour::None),
            "tile-green" => Tile::to_single(TileName::Green, 124, Behaviour::None),
            "tree-large-top" => Tile::to_single(TileName::TreeLargeTop, 90, Behaviour::None),
            "tree-large-bottom" => Tile::to_single(TileName::TreeLargeBottom, 106, Behaviour::None),
            "tree-small" => Tile::to_single(TileName::TreeSmall, 92, Behaviour::None),
            "tree-white-large-top" => {
                Tile::to_single(TileName::TreeWhiteLargeTop, 91, Behaviour::None)
            }
            "tree-white-large-bottom" => {
                Tile::to_single(TileName::TreeWhiteLargeBottom, 107, Behaviour::None)
            }
            "tree-white-small" => Tile::to_single(TileName::TreeWhiteSmall, 93, Behaviour::None),
            "tree-trunk" => Tile::to_single(TileName::TreeTrunk, 108, Behaviour::None),
            "fence" => Tile::to_single(TileName::Fence, 110, Behaviour::None),
            "bridge" => Tile::to_single(TileName::Bridge, 109, Behaviour::None),
            "bridge-rail-green" => Tile::to_single(TileName::BridgeRailGreen, 190, Behaviour::None),
            "bridge-rail-white" => Tile::to_single(TileName::BridgeRailWhite, 174, Behaviour::None),
            "waves" => Tile::to_single(TileName::Waves, 112, Behaviour::None),
            _ => Tile::to_single(TileName::Waves, 112, Behaviour::None),
        }
    }
}
