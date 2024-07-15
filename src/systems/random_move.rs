use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(MovingRandomly)]
pub fn random_move(
    ecs: &mut SubWorld,
    #[resource] map: &Map,
    #[resource] rng: &mut RandomNumberGenerator,
) {
    // Get all entities with the MovingRandomly tag
    let mut movers = <(&mut Point, &MovingRandomly)>::query();

    // Iterate over the results
    movers.iter_mut(ecs).for_each(|(pos, _)| {
        // Choose an orthogonal direction randomly
        let dest = match rng.range(0, 4) {
            0 => Point::new(-1, 0),
            1 => Point::new(1, 0),
            2 => Point::new(0, -1),
            _ => Point::new(0, 1),
        } + *pos;

        // If it's a valid destination, process it
        if map.can_enter_tile(dest) {
            *pos = dest;
        }
    });
}
