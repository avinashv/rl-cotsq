use crate::prelude::*;

#[system(for_each)]
#[read_component(Player)]
pub fn movement(
    entity: &Entity,
    want_move: &WantsToMove,
    #[resource] map: &Map,
    #[resource] camera: &mut Camera,
    ecs: &SubWorld,
    commands: &mut CommandBuffer,
) {
    // System queries for us

    // Send move command if the destination is valid
    if map.can_enter_tile(want_move.destination) {
        // Safer to use commands rather than directly modifying the component.
        // Adding a component that exists in Legion replaces the existing one.
        commands.add_component(want_move.entity, want_move.destination);

        // Is this the Player?
        if ecs
            .entry_ref(want_move.entity) // Need to access the component outside the query
            .unwrap() // Unwrap the Option
            .get_component::<Player>() // Returns a Result
            .is_ok()
        // Check the component exists
        {
            // Process Camera movement too since we know it's a Player
            camera.on_player_move(want_move.destination);
        }
    }

    // Clean up
    // // * dereferences the entity
    commands.remove(*entity);
}
