pub use crate::prelude::*;

/// Render component
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair,    // FG/BG render color
    pub glyph: FontCharType, // CP437 render glyph
}

/// Player component (tag)
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player;

/// Enemy component (tag)
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Enemy;

/// Moving Randomly component (tag)
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MovingRandomly;

/// ChasingPlayer component (tag)
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChasingPlayer;

/// WantsToMove component (intent)
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Point,
}

// WantsToAttack component (intent)
pub struct WantsToAttack {
    pub source: Entity,
    pub target: Entity,
}

/// Health component
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

/// Name component
#[derive(Clone, PartialEq)]
pub struct Name(pub String);
