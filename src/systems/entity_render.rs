use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn entity_render(ecs: &SubWorld, #[resource] camera: &Camera) {
    // Get a list of all entities to render based on fov
    let mut renderables = <(&Point, &Render)>::query();

    // Get the player's fov system
    let mut fov = <&FieldOfView>::query().filter(component::<Player>());
    let player_fov = fov.iter(ecs).nth(0).unwrap();

    // Start a new batch draw to the foreground layer
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);

    // Get the Camera offset
    let offset = Point::new(camera.left_x, camera.top_y);

    // Render anything in the player's fov
    renderables
        .iter(ecs)
        .filter(|(pos, _)| player_fov.visible_tiles.contains(&pos))
        .for_each(|(pos, render)| {
            let new_pos = *pos - offset;

            // Ensure the entities are within the viewport
            if new_pos.x > 0
                && new_pos.x < DISPLAY_WIDTH
                && new_pos.y > 0
                && new_pos.y < (DISPLAY_HEIGHT - UI_HEIGHT)
            {
                draw_batch.set(new_pos, render.color, render.glyph);
            }
        });

    // Submit the batch to the global list to process late
    draw_batch.submit(5000).expect("Entity DrawBatch error");
}
