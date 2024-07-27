use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Name)]
#[read_component(Health)]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn tooltips(ecs: &SubWorld, #[resource] mouse_pos: &Point, #[resource] camera: &Camera) {
    // Get all positions where there is an entity with a name
    let mut positions = <(Entity, &Point, &Name)>::query();

    // Get player's fov system
    let mut fov = <&FieldOfView>::query().filter(component::<Player>());
    let player_fov = fov.iter(ecs).nth(0).unwrap();

    // Get Map position with Camera offset
    let offset = Point::new(camera.left_x, camera.top_y);
    let map_pos = *mouse_pos + offset;

    // Set a new drawing batch and correct context layer
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);

    // Iterate over all the valid positions
    positions
        .iter(ecs)
        // Filter those positions which are within the map viewport that the mouse is over
        .filter(|(_, pos, _)| {
            **pos == map_pos && pos.y <= camera.bottom_y && player_fov.visible_tiles.contains(&pos)
        })
        .for_each(|(entity, _, name)| {
            let display =
                if let Ok(health) = ecs.entry_ref(*entity).unwrap().get_component::<Health>() {
                    // Display the tooltip based on if the entity has health
                    format!("{}: {} HP", &name.0, health.current)
                } else {
                    // Otherwise just the name
                    name.0.clone()
                };

            // Print at the bottom of the UI
            draw_batch.print(Point::new(0, DISPLAY_HEIGHT - (UI_HEIGHT - 2)), &display);
        });

    // Submit the batch to the global list to process quite late
    draw_batch.submit(10100).expect("Tooltip DrawBatch error.");
}
