use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    // Get all entities with a Point and filter for Player
    let mut players = <(Entity, &Point)>::query().filter(component::<Player>());

    // Check for key presses
    if let Some(key) = *key {
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

        // Iterate through all results of the filtered query
        // Should really only be one!
        players.iter_mut(ecs).for_each(|(entity, pos)| {
            // * dereferences the position
            let destination = *pos + delta;

            // Send command for WantsToMove
            // Legion's `push` needs a tuple
            commands.push((
                (),
                WantsToMove {
                    // * dereferences the entity
                    entity: *entity,
                    destination,
                },
            ));
        });

        // Change TurnState
        // * dereferences the turn state
        *turn_state = TurnState::PlayerTurn;
    }
}
