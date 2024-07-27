use crate::prelude::*;

// Create a Player entity in the ECS
pub fn spawn_player(ecs: &mut World, pos: Point) {
    // Player has tags Player, Point, Render
    ecs.push((
        Player,
        pos,
        Render {
            color: ColorPair::new(RGBA::from_u8(242, 240, 103, 255), BLACK),
            glyph: to_cp437('@'),
        },
        Health {
            current: 10,
            max: 10,
        },
        Name("Player".to_string()),
    ));
}

// Create a Monster entity in the ECS
pub fn spawn_monster(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    // Randomly choose a monster to spawn
    let (name, glyph, color, hp) = match rng.roll_dice(1, 10) {
        1..=8 => goblin(),
        _ => orc(),
    };

    // Monster has tags Enemy, Point, Render
    ecs.push((
        Enemy,
        pos,
        Render { color, glyph },
        Health {
            current: hp,
            max: hp,
        },
        Name(name),
        ChasingPlayer,
    ));
}

// Spawn a goblin
fn goblin() -> (String, FontCharType, ColorPair, i32) {
    (
        "Goblin".to_string(),
        to_cp437('g'),
        ColorPair::new(RGBA::from_u8(85, 255, 85, 255), BLACK),
        1,
    )
}

// Spawn an orc
fn orc() -> (String, FontCharType, ColorPair, i32) {
    (
        "Orc".to_string(),
        to_cp437('o'),
        ColorPair::new(RGBA::from_u8(94, 222, 143, 255), BLACK),
        2,
    )
}
