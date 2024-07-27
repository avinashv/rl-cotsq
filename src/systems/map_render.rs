use crate::prelude::*;

#[system(for_each)]
pub fn map_render(
    fov: &FieldOfView,
    _player: &Player,
    #[resource] map: &Map,
    #[resource] camera: &Camera,
) {
    // Start a new batch draw to the background layer
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);

    // Go through x and y
    for y in camera.top_y..=camera.bottom_y {
        for x in camera.left_x..camera.right_x {
            // Get the new point, the index, and the offset from the Camera
            let pt = Point::new(x, y);
            let offset = Point::new(camera.left_x, camera.top_y);
            let idx = map_idx(x, y);

            // Is it a valid point? Is it currently visible or is in player's memory?
            if map.in_bounds(pt) && (fov.visible_tiles.contains(&pt) | map.revealed_tiles[idx]) {
                // Tiles in memory are darker, in current FOV are brighter
                let tint = if fov.visible_tiles.contains(&pt) {
                    RGBA::from_u8(255, 255, 255, 255)
                } else {
                    RGBA::from_u8(76, 76, 76, 255)
                };

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
                        // Tint the walls based on visible or memory
                        draw_batch.set(pt - offset, ColorPair::new(tint, BLACK), to_cp437('#'));
                    }
                }
            }
        }
    }

    // Submit the batch to the global list to process first
    draw_batch.submit(0).expect("Map DrawBatch error.");
}
