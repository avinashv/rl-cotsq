mod camera;
mod components;
mod map;
mod map_builder;
mod spawner;
mod systems;
mod turn_state;

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
    pub use crate::turn_state::*;

    // Global constants
    pub const TILE_WIDTH: i32 = 16;
    pub const TILE_HEIGHT: i32 = TILE_WIDTH;
    pub const UI_HEIGHT: i32 = 4;
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
    input_systems: Schedule,
    player_systems: Schedule,
    monster_systems: Schedule,
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
        spawn_amulet_of_yala(&mut ecs, map_builder.amulet_start);

        // Build map
        map_builder
            .rooms
            .iter()
            .skip(1)
            .map(|r| r.center())
            .for_each(|pos| spawn_monster(&mut ecs, &mut rng, pos));

        // Inject RNG, State, Map, and Camera as resources into the ECS
        resources.insert(rng);
        resources.insert(TurnState::AwaitingInput);
        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));

        Self {
            ecs,
            resources,
            input_systems: build_input_scheduler(),
            player_systems: build_player_scheduler(),
            monster_systems: build_monster_scheduler(),
        }
    }

    /// Reset the game state cleanly
    fn reset_game_state(&mut self) {
        // Set up ECS
        self.ecs = World::default();
        self.resources = Resources::default();

        // Set up RNG and map builder
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);

        // Spawn entities
        spawn_player(&mut self.ecs, map_builder.player_start);
        spawn_amulet_of_yala(&mut self.ecs, map_builder.amulet_start);

        // Build map
        map_builder
            .rooms
            .iter()
            .skip(1)
            .map(|r| r.center())
            .for_each(|pos| spawn_monster(&mut self.ecs, &mut rng, pos));

        // Inject map, rng, camera, turn state as a Resource
        self.resources.insert(rng);
        self.resources.insert(map_builder.map);
        self.resources.insert(Camera::new(map_builder.player_start));
        self.resources.insert(TurnState::AwaitingInput);
    }

    /// Victory state
    fn victory(&mut self, ctx: &mut BTerm) {
        // Switch to the UI layer
        ctx.set_active_console(2);

        // Print the Victory message
        ctx.print_color_centered(2, RED, BLACK, "You have won!.");
        ctx.print_color_centered(4, WHITE, BLACK, "You put on the Amulet of Yala.");
        ctx.print_color_centered(5, WHITE, BLACK, "You feel it's power. You save your town.");
        ctx.print_color_centered(8, YELLOW, BLACK, "Try again with a new hero.");
        ctx.print_color_centered(9, GREEN, BLACK, "Press Space to play again.");

        // Check if the player pressed Space
        if let Some(VirtualKeyCode::Space) = ctx.key {
            // Reset the game state
            self.reset_game_state();
        }
    }

    /// Game Over state
    fn game_over(&mut self, ctx: &mut BTerm) {
        // Switch to the UI layer
        ctx.set_active_console(2);

        // Print the Game Over message
        ctx.print_color_centered(2, RED, BLACK, "Your quest has ended.");
        ctx.print_color_centered(4, WHITE, BLACK, "You were slain by a monster.");
        ctx.print_color_centered(5, WHITE, BLACK, "The Amulet of Yala remains unclaimed.");
        ctx.print_color_centered(8, YELLOW, BLACK, "Try again with a new hero.");
        ctx.print_color_centered(9, GREEN, BLACK, "Press Space to play again.");

        // Check if the player pressed Space
        if let Some(VirtualKeyCode::Space) = ctx.key {
            // Reset the game state
            self.reset_game_state();
        }
    }
}

impl GameState for State {
    /// Game loop
    fn tick(&mut self, ctx: &mut BTerm) {
        // Clear bg/fg/ui contexts
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        ctx.set_active_console(2);
        ctx.cls_bg(RGBA::from_u8(0, 0, 0, 0));

        // Inject keyboard state as a resource into the ECS
        self.resources.insert(ctx.key);

        // Inject the mouse position as a resource into the ECS
        ctx.set_active_console(0);
        self.resources.insert(Point::from_tuple(ctx.mouse_pos()));

        // Execute ECS systems based on TurnState
        // Result needs to be unwrapped (Option), clone to appease borrow checker
        let current_state = self.resources.get::<TurnState>().unwrap().clone();
        match current_state {
            TurnState::AwaitingInput => self
                .input_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::PlayerTurn => self
                .player_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::MonsterTurn => self
                .monster_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::GameOver => self.game_over(ctx),
            TurnState::Victory => self.victory(ctx),
        }

        // Render draw buffer
        render_draw_buffer(ctx).expect("Render error!");
    }
}

fn main() -> BError {
    let ctx = BTermBuilder::new()
        .with_title("Caverns of the Shadow Queen")
        .with_fps_cap(30.0)
        .with_dimensions(SCREEN_WIDTH, SCREEN_HEIGHT)
        .with_tile_dimensions(TILE_WIDTH, TILE_HEIGHT)
        .with_resource_path("res/")
        // Fonts
        .with_font(DUNGEON_FONT, TILE_WIDTH, TILE_HEIGHT)
        // Layers, 0 indexed
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, DUNGEON_FONT) // 0 - Map layer
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, DUNGEON_FONT) // 1 - Entity layer
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, DUNGEON_FONT) // 2 - UI layer
        .build()?;

    main_loop(ctx, State::new())
}
