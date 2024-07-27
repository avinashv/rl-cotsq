use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(ChasingPlayer)]
#[read_component(Health)]
#[read_component(Player)]
pub fn chasing(ecs: &SubWorld, commands: &mut CommandBuffer, #[resource] map: &Map) {
    // Query for entities that would chase, all entities with health, and the player
    let mut movers = <(Entity, &Point, &ChasingPlayer)>::query();
    let mut positions = <(Entity, &Point, &Health)>::query();
    let mut player = <(&Point, &Player)>::query();

    // Get the Point representing the player's position and its map index
    let player_pos = player.iter(ecs).nth(0).unwrap().0; // 0 is &Point
    let player_idx = map_idx(player_pos.x, player_pos.y);

    // Create a list of targets and generate a Dijkstra map for them
    let search_targets = vec![player_idx];
    let dijkstra_map = DijkstraMap::new(SCREEN_WIDTH, SCREEN_HEIGHT, &search_targets, map, 1024.0);

    // Only apply logic to any entity with ChasingPlayer
    movers.iter(ecs).for_each(|(entity, pos, _)| {
        let idx = map_idx(pos.x, pos.y);

        // Find the exit with the lowest distance to the target point
        if let Some(destination) = DijkstraMap::find_lowest_exit(&dijkstra_map, idx, map) {
            // Get the distance to the player
            let distance = DistanceAlg::Pythagoras.distance2d(*pos, *player_pos);

            // If the distance is farther away than one tile, use the dijkstra map search destination
            let destination = if distance > 1.2 {
                // 1.2 is orthogonal distance, limit to 1.42 to allow diags
                map.index_to_point2d(destination)
            } else {
                // Otherwise just set it to the player position
                *player_pos
            };

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
                    }
                    // Set the combat flag
                    attacked = true;
                });

            //  No attack, process movement
            if !attacked {
                // Sent the intent to WantsToMove
                commands.push((
                    (),
                    WantsToMove {
                        entity: *entity,
                        destination,
                    },
                ));
            }
        }
    });
}
