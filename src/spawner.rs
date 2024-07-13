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
