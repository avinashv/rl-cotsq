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
    pub fn render(&self, ctx: &mut BTerm) {
        ctx.set(
            self.position.x,
            self.position.y,
            WHITE,
            BLACK,
            to_cp437('@'),
        )
    }

    /// Handle player input
    pub fn update(&mut self, ctx: &mut BTerm, map: &Map) {
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
            }
        }
    }
}