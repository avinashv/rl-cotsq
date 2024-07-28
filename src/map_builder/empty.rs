use super::MapArchitect;
use crate::prelude::*;

pub struct EmptyArchitect {}

impl MapArchitect for EmptyArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        // Create a new MapBuilder instance to populate
        let mut mb = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            monster_spawns: Vec::new(),
            player_start: Point::zero(),
            amulet_start: Point::zero(),
        };

        // Set up map and spawns
        mb.fill(TileType::Floor); // Fill with floor
        mb.player_start = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2); // Player starts in the center
        mb.amulet_start = mb.find_most_distant();

        // Spawn 50 random monsters
        for _ in 0..50 {
            mb.monster_spawns.push(Point::new(
                rng.range(1, SCREEN_WIDTH),
                rng.range(1, SCREEN_HEIGHT),
            ));
        }

        mb
    }
}
