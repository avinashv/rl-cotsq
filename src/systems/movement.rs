use crate::prelude::*;

#[system(for_each)]
#[read_component(Player)]
#[read_component(FieldOfView)]
pub fn movement(
    entity: &Entity,
    want_move: &WantsToMove,
    #[resource] map: &mut Map,
    #[resource] camera: &mut Camera,
    ecs: &SubWorld,
    commands: &mut CommandBuffer,
) {
    // Send move command if the destination is valid
    if map.can_enter_tile(want_move.destination) {
        // Movement is possible, process it
        commands.add_component(want_move.entity, want_move.destination);

        // Is the target entity available to use?
        if let Ok(entry) = ecs.entry_ref(want_move.entity) {
            // Does the entity have a field of view?
            if let Ok(fov) = entry.get_component::<FieldOfView>() {
                // Then replace the visible set with a clone with the dirty flag
                commands.add_component(want_move.entity, fov.clone_dirty());

                // Is this the player?
                if entry.get_component::<Player>().is_ok() {
                    // Then process camera movement
                    camera.on_player_move(want_move.destination);

                    // Track revealed tiles
                    fov.visible_tiles.iter().for_each(|pos| {
                        map.revealed_tiles[map_idx(pos.x, pos.y)] = true;
                    });
                }
            }
        }
    }

    // Clean up
    commands.remove(*entity);
}
