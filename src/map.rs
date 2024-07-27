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

    /// From a given location, is the delta a valid destination?
    fn valid_exit(&self, loc: Point, delta: Point) -> Option<usize> {
        // Destination is given location plus delta
        let destination = loc + delta;

        // Is the destination on the map?
        if self.in_bounds(destination) {
            // Is it a valid tile to enter?
            if self.can_enter_tile(destination) {
                // If so, valid
                let idx = self.point2d_to_index(destination);
                Some(idx)
            } else {
                // Can't enter, invalid
                None
            }
        } else {
            // It's not on the map, invalid
            None
        }
    }
}

/// BaseMap
impl BaseMap for Map {
    /// Check any tile index for accessibility
    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        // Store a db of all possible exists for the current location
        let mut exits = SmallVec::new();
        let location = self.index_to_point2d(idx);

        // Check all available exits for the location

        // North
        if let Some(idx) = self.valid_exit(location, Point::new(-1, 0)) {
            exits.push((idx, 1.0));
        }

        // South
        if let Some(idx) = self.valid_exit(location, Point::new(1, 0)) {
            exits.push((idx, 1.0));
        }

        // West
        if let Some(idx) = self.valid_exit(location, Point::new(0, -1)) {
            exits.push((idx, 1.0));
        }

        // East
        if let Some(idx) = self.valid_exit(location, Point::new(0, 1)) {
            exits.push((idx, 1.0));
        }

        // Return valid exits
        exits
    }

    /// Return the Pythagorean distance between two indexes
    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        DistanceAlg::Pythagoras.distance2d(self.index_to_point2d(idx1), self.index_to_point2d(idx2))
    }
}

/// Algorithm2D trait for Map to enable bracket-lib pathfinding features
impl Algorithm2D for Map {
    /// Get the x and y size of the map
    fn dimensions(&self) -> Point {
        Point::new(SCREEN_WIDTH, SCREEN_HEIGHT)
    }

    /// Check whether a point is within the map bounds
    fn in_bounds(&self, point: Point) -> bool {
        self.in_bounds(point)
    }
}
