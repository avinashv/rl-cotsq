mod map;
mod map_builder;
mod player;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::player::*;

    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
}

use prelude::*;

struct State {
    map: Map,
    player: Player,
}

impl State {
    /// Constructor for new State
    fn new() -> Self {
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);

        Self {
            map: map_builder.map,
            player: Player::new(map_builder.player_start),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.player.update(ctx, &self.map);
        self.map.render(ctx);
        self.player.render(ctx);
    }
}

fn main() -> BError {
    let ctx = BTermBuilder::simple80x50()
        .with_title("Caverns of the Shadow Queen")
        .with_fps_cap(30.0)
        .build()?;

    main_loop(ctx, State::new())
}
