use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
pub fn collisions(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    // Hold player location
    let mut player_pos: Point = Point::zero();

    // Filter entities with a Point that have the Player/Enemy tag
    let mut players = <&Point>::query().filter(component::<Player>());
    let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());

    // Iterate over the results (should be one) and get the dereferenced pos
    players.iter(ecs).for_each(|pos| player_pos = *pos);

    // Iterate over enemies and remove them if their position matches Player's
    enemies
        .iter(ecs)
        .filter(|(_, pos)| **pos == player_pos) // don't need entity
        .for_each(|(entity, _)| {
            // don't need point
            commands.remove(*entity);
        });
}
