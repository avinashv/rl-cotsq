use crate::prelude::*;

mod chasing;
mod combat;
mod end_turn;
mod entity_render;
mod fov;
mod hud;
mod map_render;
mod movement;
mod player_input;
mod random_move;
mod tooltips;

/// Build schedule plan for ECS systems while AwaitingInput
pub fn build_input_scheduler() -> Schedule {
    // AwaitingInput - render maps, entities, process input
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .add_system(fov::fov_system())
        .flush() // Force above systems to complete
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(hud::hud_system())
        .add_system(tooltips::tooltips_system())
        .build()
}

/// Build schedule plan for ECS systems while PlayerTurn
pub fn build_player_scheduler() -> Schedule {
    // PlayerTurn - collisions, render, end turn
    Schedule::builder()
        .add_system(combat::combat_system())
        .flush() // Force above systems to complete
        .add_system(movement::movement_system())
        .flush() // Force above systems to complete
        .add_system(fov::fov_system())
        .flush() // Force above systems to complete
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(hud::hud_system())
        .add_system(end_turn::end_turn_system())
        .build()
}

/// Build schedule plan for ECS systems while MonsterTurn
pub fn build_monster_scheduler() -> Schedule {
    // MonsterTurn - movement, collisions, render, end turn
    Schedule::builder()
        .add_system(random_move::random_move_system())
        .add_system(chasing::chasing_system())
        .flush() // Force above systems to complete
        .add_system(combat::combat_system())
        .flush() // Force above systems to complete
        .add_system(movement::movement_system())
        .flush() // Force above systems to complete
        .add_system(fov::fov_system())
        .flush() // Force above systems to complete
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(hud::hud_system())
        .add_system(end_turn::end_turn_system())
        .build()
}
