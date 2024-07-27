use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
#[write_component(Health)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    // Get all entities with a Point and filter for Player
    let mut players = <(Entity, &Point)>::query().filter(component::<Player>());

    // Check keypresses
    if let Some(key) = key {
        // Create a new Point with the delta of movement, or zero
        let delta = match key {
            // Orthogonal directions (arrow, vi, and wasd keys)
            VirtualKeyCode::Left | VirtualKeyCode::H | VirtualKeyCode::A => Point::new(-1, 0),
            VirtualKeyCode::Right | VirtualKeyCode::L | VirtualKeyCode::D => Point::new(1, 0),
            VirtualKeyCode::Up | VirtualKeyCode::K | VirtualKeyCode::W => Point::new(0, -1),
            VirtualKeyCode::Down | VirtualKeyCode::J | VirtualKeyCode::S => Point::new(0, 1),

            // No key pressed
            _ => Point::zero(),
        };

        // Get Player and destination Entity
        let (player_entity, destination) = players
            .iter(ecs)
            .find_map(|(entity, pos)| Some((*entity, *pos + delta)))
            .unwrap();

        // Get all enemy entities
        let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());

        // Flag for waiting
        let mut did_something = false;

        // If the Player tried to move
        if delta.x != 0 || delta.y != 0 {
            // Flag for combat
            let mut hit_something = false;

            // Check if there are enemies on the destination tile
            enemies
                .iter(ecs)
                .filter(|(_, pos)| **pos == destination)
                .for_each(|(entity, _)| {
                    // Targeting an enemy on destination, so initiate combat
                    hit_something = true;

                    // Attacking is doing something
                    did_something = true;

                    // Create an attack in ECS
                    commands.push((
                        (),
                        WantsToAttack {
                            source: player_entity,
                            target: *entity,
                        },
                    ));
                });

            // No attack, so create a movement in ECS
            if !hit_something {
                // Moving is doing something
                did_something = true;

                commands.push((
                    (),
                    WantsToMove {
                        entity: player_entity,
                        destination,
                    },
                ));
            }
        }

        // If the player did nothing
        if !did_something {
            if let Ok(health) = ecs // Clippy doesn't want this mutable
                .entry_mut(player_entity)
                .unwrap()
                .get_component_mut::<Health>()
            {
                // Grant 1 hp back
                health.current = i32::min(health.max, health.current + 1);
            }
        }

        // Change TurnState
        *turn_state = TurnState::PlayerTurn;
    }
}
