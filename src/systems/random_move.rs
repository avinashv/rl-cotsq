use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(MovingRandomly)]
#[read_component(Health)]
#[read_component(Player)]
pub fn random_move(
    #[resource] rng: &mut RandomNumberGenerator,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    // Get all entities with the MovingRandomly tag
    let mut movers = <(Entity, &Point, &MovingRandomly)>::query();

    // Get all entities with the Health tag
    let mut positions = <(Entity, &Point, &Health)>::query();

    // Iterate over the results
    movers.iter(ecs).for_each(|(entity, pos, _)| {
        // Choose an orthogonal direction randomly
        let destination = match rng.range(0, 4) {
            0 => Point::new(-1, 0),
            1 => Point::new(1, 0),
            2 => Point::new(0, -1),
            _ => Point::new(0, 1),
        } + *pos;

        // Combat tracking flag
        let mut attacked = false;

        // Filter through all positions for the target destination that includes a Player
        positions
            .iter(ecs)
            .filter(|(_, target_pos, _)| **target_pos == destination)
            .for_each(|(victim, _, _)| {
                if ecs
                    .entry_ref(*victim)
                    .unwrap()
                    .get_component::<Player>()
                    .is_ok()
                {
                    // If it's the Player, attack
                    commands.push((
                        (),
                        WantsToAttack {
                            source: *entity,
                            target: *victim,
                        },
                    ));

                    // Set the combat flag
                    attacked = true;
                }
            });

        // No attack, so process movement
        if !attacked {
            // Send the command for WantsToMove
            commands.push((
                (),
                WantsToMove {
                    entity: *entity,
                    destination,
                },
            ));
        }
    });
}
