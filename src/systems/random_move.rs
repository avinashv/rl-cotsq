use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(MovingRandomly)]
#[read_component(FieldOfView)]
#[read_component(Health)]
#[read_component(Player)]
pub fn random_move(
    #[resource] rng: &mut RandomNumberGenerator,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    // Get all entities with the MovingRandomly tag
    let mut movers = <(Entity, &Point, &MovingRandomly, &FieldOfView)>::query();

    // Get all entities with the Health tag
    let mut positions = <(Entity, &Point, &Health)>::query();

    // Get the player position to check if its visible in FOV
    let mut player = <(&Point, &Player)>::query();
    let player_pos = player.iter(ecs).nth(0).unwrap().0; // 0 is &Point

    // Iterate over the results
    movers.iter(ecs).for_each(|(entity, pos, _, fov)| {
        // If the monster can see the player, stop moving randomly
        if fov.visible_tiles.contains(&player_pos) {
            return;
        }

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