mod map;
mod map_builder;
mod player;
mod camera;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::player::*;
    pub use crate::camera::*;

    pub const TILE_WIDTH: i32 = 16;
    pub const TILE_HEIGHT: i32 = TILE_WIDTH;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub const DUNGEON_FONT: &str = "Anikki_square_16x16.png";
}

use prelude::*;

struct State {
    map: Map,
    player: Player,
    camera: Camera,
}

impl State {
    /// Constructor for new State
    fn new() -> Self {
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);

        Self {
            map: map_builder.map,
            player: Player::new(map_builder.player_start),
            camera: Camera::new(map_builder.player_start),
        }
    }
}

impl GameState for State {
    /// Game loop
    fn tick(&mut self, ctx: &mut BTerm) {
        // Clear bg/fg contexts
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();

        self.player.update(ctx, &self.map, &mut self.camera);
        self.map.render(ctx, &self.camera);
        self.player.render(ctx, &self.camera);
    }
}

fn main() -> BError {
    let ctx = BTermBuilder::new()
        .with_title("Caverns of the Shadow Queen")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(TILE_WIDTH, TILE_HEIGHT)
        .with_resource_path("res/")
        .with_font(DUNGEON_FONT, TILE_WIDTH, TILE_HEIGHT)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, DUNGEON_FONT)
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, DUNGEON_FONT)
        .build()?;

    main_loop(ctx, State::new())
}
