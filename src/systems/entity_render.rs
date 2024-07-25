use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
pub fn entity_render(ecs: &SubWorld, #[resource] camera: &Camera) {
    // Start a new batch draw to the foreground layer
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);

    // Get the Camera offset
    let offset = Point::new(camera.left_x, camera.top_y);

    // Get all the entities with a Point and Render component
    <(&Point, &Render)>::query()
        .iter(ecs)
        .for_each(|(pos, render)| {
            let new_pos = *pos - offset;
            // Ensure the entities are within the viewport
            if new_pos.x > 0
                && new_pos.x < DISPLAY_WIDTH
                && new_pos.y > 0
                && new_pos.y < (DISPLAY_HEIGHT - UI_HEIGHT)
            {
                // Iterate through each and add them to the draw batch
                draw_batch.set(new_pos, render.color, render.glyph);
            }
        });

    // Submit the batch to the global list to process late
    draw_batch.submit(5000).expect("Entity DrawBatch error");
}