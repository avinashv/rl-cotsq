pub use crate::prelude::*;
use std::collections::HashSet;

/// Render component
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair,    // FG/BG render color
    pub glyph: FontCharType, // CP437 render glyph
}

/// Field of View component
#[derive(Clone, Debug, PartialEq)]
pub struct FieldOfView {
    pub visible_tiles: HashSet<Point>, // Tiles visible to the entity
    pub radius: i32,                   // Field of view range
    pub is_dirty: bool,                // Dirty FoV needs updating
}

impl FieldOfView {
    /// Initialize a new field of view
    pub fn new(radius: i32) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            radius,
            is_dirty: true, // process the fov on creation
        }
    }

    /// Create a perfect copy of the field of view set to dirty
    pub fn clone_dirty(&self) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            radius: self.radius,
            is_dirty: true,
        }
    }
}

/// Player component (tag)
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player;

/// Enemy component (tag)
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Enemy;

/// Item component (tag)
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Item;

/// Amulet of Yala component (tag)
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AmuletOfYala;

/// Moving Randomly component (tag)
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MovingRandomly;

/// Chasing Player component (tag)
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
