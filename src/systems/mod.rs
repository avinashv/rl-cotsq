use crate::prelude::*;

mod collisions;
mod end_turn;
mod entity_render;
mod map_render;
mod player_input;
mod random_move;

/// Build schedule plan for ECS systems while AwaitingInput
pub fn build_input_scheduler() -> Schedule {
    // AwaitingInput - render maps, entities, process input
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .flush() // Force above systems to complete
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .build()
}

/// Build schedule plan for ECS systems while PlayerTurn
pub fn build_player_scheduler() -> Schedule {
    // PlayerTurn - collisions, render, end turn
    Schedule::builder()
        .add_system(collisions::collisions_system())
        .flush() // Force above systems to complete
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(end_turn::end_turn_system())
        .build()
}

/// Build schedule plan for ECS systems while MonsterTurn
pub fn build_monster_scheduler() -> Schedule {
    // MonsterTurn - movement, collisions, render, end turn
    Schedule::builder()
        .add_system(random_move::random_move_system())
        .flush() // Force above systems to complete
        .add_system(collisions::collisions_system())
        .flush() // Force above systems to complete
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(end_turn::end_turn_system())
        .build()
}
