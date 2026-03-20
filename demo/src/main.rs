use engine::{
    entity_manager::{Entity, EntityId, Position},
    errors::GameError,
    game_state::{Action, GameState},
};

fn main() -> Result<(), GameError> {
    let mut state = setup_duel(4524625);

    while !is_finished(&state) {
        let action = choose_action(&state)?;
        state.step(action)?;
        print_state(&state);
    }

    println!("winner: {:?}", winner(&state));
    Ok(())
}

fn setup_duel(seed: u64) -> GameState {
    let mut state = GameState::new(seed);

    state.entities.push(Entity::new(1, 10, 100, 1, 1));

    state.entities.push(Entity::new(2, 100, 10, 8, 1));

    state.current_actor = 1;

    state
}

fn is_finished(state: &GameState) -> bool {
    state.entities.iter().filter(|e| !e.is_dead).count() <= 1
}

fn winner(state: &GameState) -> Option<EntityId> {
    state.entities.iter().find(|e| !e.is_dead).map(|e| e.id)
}

fn choose_action(state: &GameState) -> Result<Action, GameError> {
    let actor = state.current_actor;

    let me = state
        .entities
        .iter()
        .find(|e| e.id == actor)
        .ok_or(GameError::EntityNotFound(actor))?;

    let target = state
        .entities
        .iter()
        .find(|e| !e.is_dead && e.id != actor)
        .ok_or(GameError::EntityNotFound(actor))?;

    let dist = me.position.calculate_manhattan_distance(&target.position);

    if me.stats.ap < 4 {
        return Ok(Action::Wait { actor });
    }

    if dist <= 2 {
        return Ok(Action::Attack {
            actor,
            target: target.id,
        });
    }

    let next = step_towards(&me.position, &target.position);

    Ok(Action::Move {
        actor,
        position: next,
    })
}

fn step_towards(from: &Position, to: &Position) -> Position {
    let mut x = from.x;
    let mut y = from.y;

    if x < to.x {
        x += 1;
    } else if x > to.x {
        x -= 1;
    } else if y < to.y {
        y += 1;
    } else if y > to.y {
        y -= 1;
    }

    Position { x, y }
}

fn print_state(state: &GameState) {
    println!("tick: {}", state.tick);

    for e in &state.entities {
        println!(
            "entity={} pos=({}, {}) hp={} ap={} dead={}",
            e.id, e.position.x, e.position.y, e.stats.hp, e.stats.ap, e.is_dead
        );
    }

    println!("current_actor={}", state.current_actor);
    println!("hash={}", state.hash());
    println!("---");
}
