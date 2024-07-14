use crate::prelude::*;

/// Spawn player to the given position
pub fn spawn_player(ecs: &mut World, pos: Point) {
    // Create a new Player entity
    ecs.push((
        Player, // Player tag
        pos,    // Position component (Point)
        Render {
            // Render component
            color: ColorPair::new(RGBA::from_u8(242, 240, 103, 255), BLACK),
            glyph: to_cp437('@'),
        },
    ));
}

/// Spawn a random monster to the given position
pub fn spawn_monster(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    // Create a new monster entity
    ecs.push((
        Enemy, // Enemy tag
        pos,   // Position component (Point)
        Render {
            // Render component
            color: ColorPair::new(RGBA::from_u8(255, 85, 85, 255), BLACK),
            glyph: match rng.range(0, 4) {
                0 => to_cp437('E'),
                1 => to_cp437('O'),
                2 => to_cp437('o'),
                _ => to_cp437('g'),
            },
        },
    ));
}
