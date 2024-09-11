use bevy::prelude::*;

pub enum Direction {
    Right,
    TopRight,
    Top,
    TopLeft,
    Left,
    BottomLeft,
    Bottom,
    BottomRight,
    Neutral,
}

impl From<Vec2> for Direction {
    fn from(value: Vec2) -> Self {
        if value.x > 0. && value.y > 0. {
            Self::TopRight
        } else if value.x < 0. && value.y > 0. {
            Self::TopLeft
        } else if value.x < 0. && value.y < 0. {
            Self::BottomLeft
        } else if value.x > 0. && value.y < 0. {
            Self::BottomRight
        } else if value.x > 0. {
            Self::Right
        } else if value.x < 0. {
            Self::Left
        } else if value.y > 0. {
            Self::Top
        } else if value.y < 0. {
            Self::Bottom
        } else {
            Self::Neutral
        }
    }
}
