use crate::prelude::*;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

/// Calculate the index in a 1d vector given x, y coordinates.
pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}

/// Represents a preset list of tile types in a map.
#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

/// Map structure made up of a list of TileTypes.
pub struct Map {
    pub tiles: Vec<TileType>,
}

impl Map {
    /// Constructor for a new map
    pub fn new() -> Self {
        Self {
            // Single-dimensioned vector of floor tiles.
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    /// Render the map to the screen
    pub fn render(&self, ctx: &mut BTerm, camera: &Camera) {
        // Render to the background layer
        ctx.set_active_console(0);

        // Go through x, y
        for y in camera.top_y..camera.bottom_y {
            for x in camera.left_x..camera.right_x {
                if self.in_bounds(Point::new(x, y)) {
                    // Determine the index
                    let idx = map_idx(x, y);

                    // Check if its Floor or Wall and draw it appropriately
                    match self.tiles[idx] {
                        TileType::Floor => {
                            ctx.set(x - camera.left_x, y - camera.top_y, RGBA::from_u8(76, 76, 76, 255), BLACK, to_cp437('.'));
                        }
                        TileType::Wall => {
                            ctx.set(x - camera.left_x, y - camera.top_y, WHITE, BLACK, to_cp437('#'));
                        }
                    }
                }
            }
        }
    }

    /// Check whether a Point is within the Map
    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    /// Check whether a Point is "walkable" within the Map
    pub fn can_enter_tile(&self, point: Point) -> bool {
        // A walkable tile is a Floor and in bounds
        self.in_bounds(point) && self.tiles[map_idx(point.x, point.y)] == TileType::Floor
    }

    /// Determine a tile's index, indicate an error if it's outside of bounds
    pub fn try_idx(&self, point: Point) -> Option<usize> {
        if !self.in_bounds(point) {
            None
        } else {
            Some(map_idx(point.x, point.y))
        }
    }
}