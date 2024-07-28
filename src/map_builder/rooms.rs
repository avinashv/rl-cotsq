use crate::prelude::*;

use super::MapArchitect;

pub struct RoomsArchitect {}

impl MapArchitect for RoomsArchitect {
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
        mb.fill(TileType::Wall); // Fill with walls
        mb.build_random_rooms(rng); // Carve rooms
        mb.build_corridors(rng); // Carve corridors
        mb.player_start = mb.rooms[0].center(); // Player starts in the center of first room
        mb.amulet_start = mb.find_most_distant(); // Amulet is as far as possible from player

        // Spawn a monster per room except the player's start
        for room in mb.rooms.iter().skip(1) {
            mb.monster_spawns.push(room.center());
        }
        mb
    }
}
