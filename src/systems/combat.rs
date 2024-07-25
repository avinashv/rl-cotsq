use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
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
    targets.iter().for_each(|(message, target)| {
        // target must have health to be in combat
        if let Ok(health) = ecs
            .entry_mut(*target)
            .unwrap()
            .get_component_mut::<Health>()
        {
            // TODO Debug
            println!("HP before {}", health.current);

            // Target takes damage
            health.current -= 1;

            // If the target doesn't have health, remove it
            if health.current < 1 {
                commands.remove(*target);
            }

            // TODO Debug
            println!("Health after {}", health.current);
        }

        // Remove WantsToAttack intent message
        commands.remove(*message);
    });
}
