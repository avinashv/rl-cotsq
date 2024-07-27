use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
#[read_component(Player)]
#[write_component(Health)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    // Get the list of potential attack sources
    let mut sources = <(Entity, &WantsToAttack)>::query();

    // Build a list of targets
    let targets: Vec<(Entity, Entity)> = sources
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.target))
        .collect();

    // Process combat for each target
    targets
        .iter()
        .for_each(|(message, target)| {
            // Get the player entity
            let is_player = ecs
                .entry_ref(*target)
                .unwrap()
                .get_component::<Player>()
                .is_ok();

            // Target must have health to be in combat
            if let Ok(health) = ecs // Clippy doesn't want this mutable
                .entry_mut(*target)
                .unwrap()
                .get_component_mut::<Health>()
            {
                // Target takes damage
                health.current -= 1;

                // If the target doesn't have health and is *not* the player, remove it
                if health.current < 1 && !is_player {
                    commands.remove(*target);
                }
            }

            // Remove WantsToAttack intent message
            commands.remove(*message);
        });
}