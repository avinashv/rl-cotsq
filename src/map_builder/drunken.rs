use crate::prelude::*;

use super::MapArchitect;

const WALK_DISTANCE: usize = 400;
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;
const WANTED_FLOOR: usize = NUM_TILES / 3;

pub struct DrunkenWalkArchitect {}

impl MapArchitect for DrunkenWalkArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        // Create a new MapBuilder instance to populate
        let mut mb = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            monster_spawns: Vec::new(),
            player_start: Point::zero(),
            amulet_start: Point::zero(),
        };

        // Set up map, center, and the first walker
        mb.fill(TileType::Wall);
        let center = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
        self.drunken_walk(&center, rng, &mut mb.map);

        // While the number of wanted floor tiles is not met
        while mb
            .map
            .tiles
            .iter()
            .filter(|t| **t == TileType::Floor)
            .count()
            < WANTED_FLOOR
        {
            // Add another walker
            self.drunken_walk(
                &Point::new(rng.range(0, SCREEN_WIDTH), rng.range(0, SCREEN_HEIGHT)),
                rng,
                &mut mb.map,
            );

            // Build a dijkstra map of every tile from the center
            let dijkstra_map = DijkstraMap::new(
                SCREEN_WIDTH,
                SCREEN_HEIGHT,
                &vec![mb.map.point2d_to_index(center)],
                &mb.map,
                1024.0,
            );

            // Go through the dijkstra map and turn inaccessible tiles into walls
            dijkstra_map
                .map
                // Iterate through the map
                .iter()
                // Enumerate adding an index
                .enumerate()
                // Filter for tiles more than 2000 distance away
                .filter(|(_, distance)| *distance > &2000.0)
                // Convert them to wall tiles
                .for_each(|(idx, _)| mb.map.tiles[idx] = TileType::Wall);
        }

        // Set up spawns
        mb.monster_spawns = mb.spawn_monsters(&center, rng);
        mb.player_start = center;
        mb.amulet_start = mb.find_most_distant();

        mb
    }
}

impl DrunkenWalkArchitect {
    fn drunken_walk(&mut self, start: &Point, rng: &mut RandomNumberGenerator, map: &mut Map) {
        // Set up start position. Drunk hasn't walked any steps yet
        let mut drunk_pos = start.clone();
        let mut distance_walked = 0;

        // Loop forever
        loop {
            // Set the current position to Floor
            let drunk_idx = map.point2d_to_index(drunk_pos);
            map.tiles[drunk_idx] = TileType::Floor;

            // Randomly assign the next direction
            match rng.range(0, 4) {
                0 => drunk_pos.x -= 1,
                1 => drunk_pos.x += 1,
                2 => drunk_pos.y -= 1,
                _ => drunk_pos.y += 1,
            }

            // If the walker leaves the map, break
            if !map.in_bounds(drunk_pos) {
                break;
            }

            // Increment total steps, and if it goes over, break
            distance_walked += 1;
            if distance_walked > WALK_DISTANCE {
                break;
            }
        }
    }
}
