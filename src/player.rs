use crate::prelude::*;

pub struct Player {
    pub position: Point,
}

impl Player {
    /// Constructor for a new player
    pub fn new(position: Point) -> Self {
        Self { position }
    }

    /// Render the player to the screen
    pub fn render(&self, ctx: &mut BTerm, camera: &Camera) {
        // Render to the foreground layer
        ctx.set_active_console(1);

        // Draw the player
        ctx.set(
            self.position.x - camera.left_x,
            self.position.y - camera.top_y,
            RGBA::from_u8(242, 240, 103, 255),
            BLACK,
            to_cp437('@'),
        );
    }

    /// Handle player input
    pub fn update(&mut self, ctx: &mut BTerm, map: &Map, camera: &mut Camera) {
        // Check input events to create direction delta
        if let Some(key) = ctx.key {
            let delta = match key {
                VirtualKeyCode::Left => Point::new(-1, 0),
                VirtualKeyCode::Right => Point::new(1, 0),
                VirtualKeyCode::Up => Point::new(0, -1),
                VirtualKeyCode::Down => Point::new(0, 1),
                _ => Point::zero(),
            };

            // If the new position is legal, process it
            let new_position = self.position + delta;
            if map.can_enter_tile(new_position) {
                self.position = new_position;
                camera.on_player_move(new_position);
            }
        }
    }
}