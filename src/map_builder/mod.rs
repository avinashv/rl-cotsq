use crate::prelude::*;

mod empty;
mod rooms;

// use empty::EmptyArchitect;
use rooms::RoomsArchitect;

const NUM_ROOMS: usize = 20;

trait MapArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder;
}

/// Builder structure to hold its map, rooms, and player start.
pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub monster_spawns: Vec<Point>,
    pub player_start: Point,
    pub amulet_start: Point,
}

impl MapBuilder {
    pub fn new(rng: &mut RandomNumberGenerator) -> Self {
        let mut architect = RoomsArchitect {};
        // let mut architect = EmptyArchitect {};
        architect.new(rng)
    }

    /// Fill a map entirely with the chosen TileType
    fn fill(&mut self, tile: TileType) {
        // change each tile to a wall
        // * needs to dereference to change the actual tile
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    /// Find the most distant tile from the player start tile
    fn find_most_distant(&self) -> Point {
        // Create a dijkstra map using the player's start (map start)
        let dijkstra_map = DijkstraMap::new(
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            &vec![self.map.point2d_to_index(self.player_start)],
            &self.map,
            1024.0,
        );

        // Make a large number and use this to flag unreachable tiles in the dijkstra map
        const UNREACHABLE: &f32 = &f32::MAX;

        // Wrap logic directly to the index of the amulet
        self.map.index_to_point2d(
            dijkstra_map
                .map
                .iter()
                // Enum adds an index to each entry, returning tuple (index, distance)
                .enumerate()
                // Remove any tiles that are unreachable
                .filter(|(_, dist)| *dist < UNREACHABLE)
                // Look for the most distant tile
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap()) // unwrap because partial_cmp returns an option
                .unwrap()
                .0, // unwrap because max_by returns an option
        )
    }

    /// Create rooms with random size and location if they don't overlap
    fn build_random_rooms(&mut self, rng: &mut RandomNumberGenerator) {
        // Iterate through the list of existing rooms
        while self.rooms.len() < NUM_ROOMS {
            // Create a randomly sized and placed room
            let room = Rect::with_size(
                rng.range(1, SCREEN_WIDTH - 10),
                rng.range(1, SCREEN_HEIGHT - 10),
                rng.range(2, 10),
                rng.range(2, 10),
            );

            // Flag to track if it overlaps
            let mut overlap = false;

            // Iterate over existing rooms and flag on overlap
            for r in self.rooms.iter() {
                if r.intersect(&room) {
                    overlap = true;
                }
            }

            // If it didn't overlap
            if !overlap {
                // Carve out each tile of the room size on the map into a Floor
                room.for_each(|rr| {
                    if rr.x > 0 && rr.x < SCREEN_WIDTH && rr.y > 0 && rr.y < SCREEN_HEIGHT {
                        let idx = map_idx(rr.x, rr.y);
                        self.map.tiles[idx] = TileType::Floor;
                    }
                });

                // Put the room onto the list
                self.rooms.push(room);
            }
        }
    }

    /// Carve a vertical tunnel
    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        use std::cmp::{max, min};

        // Iterate from the lowest to the highest position
        for y in min(y1, y2)..=max(y1, y2) {
            // If the current coordinate is a safe index
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                // Carve it into a Floor
                self.map.tiles[idx] = TileType::Floor;
            }
        }
    }

    /// Carve a horizontal tunnel
    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        use std::cmp::{max, min};

        // Iterate from the leftmost to the rightmost position
        for x in min(x1, x2)..=max(x1, x2) {
            // If the current coordinate is a safe index
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                // Carve it into a Floor
                self.map.tiles[idx] = TileType::Floor;
            }
        }
    }

    /// Carves tunnels between rooms
    fn build_corridors(&mut self, rng: &mut RandomNumberGenerator) {
        // Create a safe copy of the rooms list to manipulate
        let mut rooms = self.rooms.clone();

        // Sort the rooms by x center to build shorter corridors
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));

        // Enumerate over the rooms
        // Skip the first one because we don't need to tunnel from anywhere yet
        for (i, room) in rooms.iter().enumerate().skip(1) {
            // i is counter from enumerate

            // Track old and new room to tunnel to and from
            let prev = rooms[i - 1].center();
            let new = room.center();

            // Randomly choose which direction to tunnel first, and then elbow
            if rng.range(0, 2) == 1 {
                self.apply_horizontal_tunnel(prev.x, new.x, prev.y);
                self.apply_vertical_tunnel(prev.y, new.y, new.x);
            } else {
                self.apply_vertical_tunnel(prev.y, new.y, prev.x);
                self.apply_horizontal_tunnel(prev.x, new.x, new.y);
            }
        }
    }
}
