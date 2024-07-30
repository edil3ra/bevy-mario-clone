use serde::Deserialize;

#[derive(Debug, Hash, Deserialize, PartialEq, Eq)]
pub enum Pattern {
    Castle,
    Overworld,
    Underwater,
    Underworld,
}

impl From<&str> for Pattern {
    fn from(value: &str) -> Self {
        match value {
            "overworld.pattern" => Self::Overworld,
            "castle.pattern" => Self::Castle,
            "underwater.pattern" => Self::Underwater,
            "underworld.pattern" => Self::Underworld,
            _ => Self::Overworld,
        }
    }
}
