mod camera;
mod components;
mod map;
mod map_builder;
mod spawner;
mod systems;

mod prelude {
    // External crates
    pub use bracket_lib::prelude::*;
    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use legion::*;

    // Internal modules (global namespace)
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;

    // Global constants
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
    ecs: World,
    resources: Resources,
    systems: Schedule,
}

impl State {
    /// Constructor for new State
    fn new() -> Self {
        // Set up ECS
        let mut ecs = World::default();
        let mut resources = Resources::default();

        // Set up RNG and MapBuilder
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);

        // Spawn entities
        spawn_player(&mut ecs, map_builder.player_start);
        map_builder
            .rooms
            .iter()
            .skip(1)
            .map(|r| r.center())
            .for_each(|pos| spawn_monster(&mut ecs, &mut rng, pos));

        // Inject RNG, Map, and Camera as resources into the ECS
        resources.insert(rng);
        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));

        Self {
            ecs,
            resources,
            systems: build_scheduler(),
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

        // Inject keyboard state as a resource into the ECS
        self.resources.insert(ctx.key);

        // Execute systems
        self.systems.execute(&mut self.ecs, &mut self.resources);

        // Render draw buffer
        render_draw_buffer(ctx).expect("Render error!");
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
