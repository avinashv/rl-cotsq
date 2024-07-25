use crate::prelude::*;

#[system]
pub fn map_render(#[resource] map: &Map, #[resource] camera: &Camera) {
    // Start a new batch draw to the background layer
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);

    // Go through x and y
    for y in camera.top_y..=camera.bottom_y {
        for x in camera.left_x..camera.right_x {
            // Get the new point and the offset from the Camera
            let pt = Point::new(x, y);
            let offset = Point::new(camera.left_x, camera.top_y);

            // Is it a valid point?
            if map.in_bounds(pt) {
                // Determine the index
                let idx = map_idx(x, y);

                // Check if it's a Floor or Tile and draw it appropriately
                match map.tiles[idx] {
                    TileType::Floor => {
                        draw_batch.set(
                            pt - offset,
                            ColorPair::new(RGBA::from_u8(76, 76, 76, 255), BLACK),
                            to_cp437('.'),
                        );
                    }
                    TileType::Wall => {
                        draw_batch.set(
                            pt - offset,
                            ColorPair::new(RGBA::from_u8(187, 187, 187, 255), BLACK),
                            to_cp437('#'),
                        );
                    }
                }
            }
        }
    }

    // Submit the batch to the global list to process first
    draw_batch.submit(0).expect("Map DrawBatch error.");
}
