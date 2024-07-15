use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(MovingRandomly)]
pub fn random_move(
    #[resource] rng: &mut RandomNumberGenerator,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    // Get all entities with the Point component and MovingRandomly tag
    let mut movers = <(Entity, &Point, &MovingRandomly)>::query();

    // Iterate over the results
    // We don't need to iterate over the MovingRandomly tag
    movers.iter(ecs).for_each(|(entity, pos, _)| {
        // Choose an orthogonal direction randomly
        let destination = match rng.range(0, 4) {
            0 => Point::new(-1, 0),
            1 => Point::new(1, 0),
            2 => Point::new(0, -1),
            _ => Point::new(0, 1),
        } + *pos; // * dereferences the position

        // Send the command for WantsToMove
        commands.push((
            (),
            WantsToMove {
                // * dereferences the entity
                entity: *entity,
                destination,
            },
        ));
    });
}
