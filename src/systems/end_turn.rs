use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Point)]
#[read_component(AmuletOfYala)]
pub fn end_turn(ecs: &SubWorld, #[resource] turn_state: &mut TurnState) {
    let mut player_hp = <(&Health, &Point)>::query().filter(component::<Player>());
    let mut amulet = <&Point>::query().filter(component::<AmuletOfYala>());

    let current_state = turn_state.clone();

    // New turn state depends on the old
    let mut new_state = match current_state {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
        _ => current_state,
    };

    // Get amulet position
    let amulet_pos = amulet.iter(ecs).nth(0).unwrap();

    // Check game end conditions
    player_hp.iter(ecs).for_each(|(hp, pos)| {
        // If the player doesn't have health, switch to GameOver
        if hp.current < 1 {
            new_state = TurnState::GameOver;
        }

        // If the player is at the amulet, switch to Victory
        if pos == amulet_pos {
            new_state = TurnState::Victory;
        }
    });

    // Process the new state
    *turn_state = new_state;
}
