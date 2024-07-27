use crate::prelude::*;

#[system(for_each)]
#[read_component(Player)]
#[read_component(Health)]
pub fn hud(health: &Health, _player: &Player, _ecs: &SubWorld) {
    // Create a new drawing batch
    let mut draw_batch = DrawBatch::new();

    // Set the context to the UI layer
    draw_batch.target(2);

    // Draw the health bar
    draw_batch.bar_horizontal(
        Point::new(0, DISPLAY_HEIGHT - UI_HEIGHT),
        DISPLAY_WIDTH,
        health.current,
        health.max,
        ColorPair::new(RED, BLACK),
    );

    // Draw the HP
    draw_batch.print_color(
        Point::new(0, DISPLAY_HEIGHT - UI_HEIGHT + 1),
        format!("HP {}/{} ", health.current, health.max),
        ColorPair::new(WHITE, BLACK),
    );

    draw_batch.submit(10000).expect("UI DrawBatch Error.");
}
