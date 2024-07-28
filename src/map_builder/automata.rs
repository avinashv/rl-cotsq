use crate::prelude::*;

use super::MapArchitect;

pub struct CellularAutomataArchitect {}

impl CellularAutomataArchitect {
    // Generate random noise, slightly biased towards Floor
    fn random_noise_map(&mut self, rng: &mut RandomNumberGenerator, map: &mut Map) {
        // Go through each tile
        map.tiles.iter_mut().for_each(|t| {
            // Get a random number
            let roll = rng.range(0, 100);

            if roll > 55 {
                // 55% of the tile floor
                *t = TileType::Floor;
            } else {
                // Otherwise wall
                *t = TileType::Wall;
            }
        });
    }

    // Return the number of neighbors of a tile that are walls
    fn count_neighbors(&self, x: i32, y: i32, map: &Map) -> usize {
        let mut neighbors = 0;

        // Check each orthogonal and diagonal coordinate around the point
        for iy in -1..=1 {
            for ix in -1..=1 {
                // Skip the actual point and count walls
                if !(ix == 0 && iy == 0) && map.tiles[map_idx(x + ix, y + iy)] == TileType::Wall {
                    neighbors += 1;
                }
            }
        }

        neighbors
    }

    // Run cellular automata iterations
    fn iteration(&mut self, map: &mut Map) {
        // Create a copy of the tiles to effect
        let mut new_tiles = map.tiles.clone();

        // Iterate through the tiles, skipping edges
        for y in 1..SCREEN_HEIGHT - 1 {
            for x in 1..SCREEN_WIDTH - 1 {
                // Get the current index and count neighbors
                let idx = map_idx(x, y);
                let neighbors = self.count_neighbors(x, y, map);

                // Rules:
                // 0 or 4 neighors: wall
                // otherwise: floor
                if neighbors > 4 || neighbors == 0 {
                    new_tiles[idx] = TileType::Wall;
                } else {
                    new_tiles[idx] = TileType::Floor;
                }
            }
        }

        // Replace the tiles with the new set after iteration
        map.tiles = new_tiles;
    }

    // Find the closest valid start from the center of the map
    fn find_player_start(&self, map: &Map) -> Point {
        // Store the center of the map
        let center = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);

        let closest_point = map
            .tiles
            // Iterate over all the tiles
            .iter()
            // Add index, type is now (index, tiletype)
            .enumerate()
            // Remove all the tiles that aren't a floor
            .filter(|(_, t)| **t == TileType::Floor)
            // Calculate the Pythagorean distance for each remaining tile to the center
            .map(|(idx, _)| {
                (
                    idx,
                    DistanceAlg::Pythagoras.distance2d(center, map.index_to_point2d(idx)),
                )
            })
            // Find the minimum comparing the distance to the center
            .min_by(|(_, distance), (_, distance2)| distance.partial_cmp(&distance2).unwrap())
            // Transform the tuple into just the index
            .map(|(idx, _)| idx)
            .unwrap();

        // Return the index as (x, y)
        map.index_to_point2d(closest_point)
    }
}

impl MapArchitect for CellularAutomataArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        // Create a new MapBuilder instance to populate
        let mut mb = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            monster_spawns: Vec::new(),
            player_start: Point::zero(),
            amulet_start: Point::zero(),
        };

        // Set up map
        self.random_noise_map(rng, &mut mb.map);
        for _ in 0..10 {
            // Run 10 iterations
            self.iteration(&mut mb.map);
        }

        // Make the border of the map all walls
        for x in 0..SCREEN_WIDTH {
            mb.map.tiles[map_idx(x, 0)] = TileType::Wall;
            mb.map.tiles[map_idx(x, SCREEN_HEIGHT - 1)] = TileType::Wall;
        }
        for y in 0..SCREEN_HEIGHT {
            mb.map.tiles[map_idx(0, y)] = TileType::Wall;
            mb.map.tiles[map_idx(SCREEN_WIDTH - 1, y)] = TileType::Wall;
        }

        // Set up spawns
        let start = self.find_player_start(&mb.map);
        mb.player_start = start;
        mb.amulet_start = mb.find_most_distant();
        mb.monster_spawns = mb.spawn_monsters(&start, rng);

        mb
    }
}
