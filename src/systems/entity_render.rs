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
            // Iterate through each and add them to the draw batch
            // * dereferences the position
            draw_batch.set(*pos - offset, render.color, render.glyph);
        });

    // Submit the batch to the global list to process late
    draw_batch.submit(5000).expect("Batch error");
}
