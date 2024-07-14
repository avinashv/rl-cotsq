use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(Player)]
pub fn player_input(
    ecs: &mut SubWorld,
    #[resource] map: &Map,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] camera: &mut Camera,
) {
    if let Some(key) = key {
        // Create a new Point with the delta of movement, or zero
        let delta = match key {
            // Orthogonal directions (arrow, vi, and wasd keys)
            VirtualKeyCode::Left | VirtualKeyCode::H | VirtualKeyCode::A => Point::new(-1, 0),
            VirtualKeyCode::Right | VirtualKeyCode::L | VirtualKeyCode::D => Point::new(1, 0),
            VirtualKeyCode::Up | VirtualKeyCode::K | VirtualKeyCode::W => Point::new(0, -1),
            VirtualKeyCode::Down | VirtualKeyCode::J | VirtualKeyCode::S => Point::new(0, 1),

            // Diagonal directors (vi and wasd keys)
            VirtualKeyCode::Y | VirtualKeyCode::Q => Point::new(-1, -1),
            VirtualKeyCode::U | VirtualKeyCode::E => Point::new(1, -1),
            VirtualKeyCode::N | VirtualKeyCode::C => Point::new(1, 1),
            VirtualKeyCode::B | VirtualKeyCode::Z => Point::new(-1, 1),

            // No key pressed
            _ => Point::zero(),
        };

        // If there was any change in position
        if delta.x != 0 || delta.y != 0 {
            // Get all entities with a Point and filter for Player
            let mut players = <&mut Point>::query().filter(component::<Player>());

            // Iterate through all results of the filtered query. Should really only be one!
            players.iter_mut(ecs).for_each(|pos| {
                // * dereferences the position
                let dest = *pos + delta;

                // Only process the movement if it's valid
                if map.can_enter_tile(dest) {
                    *pos = dest;
                    camera.on_player_move(dest);
                }
            });
        }
    }
}
