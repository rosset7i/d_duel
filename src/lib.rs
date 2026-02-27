use rand::{Rng, SeedableRng, rngs::StdRng};

struct ActionError;

struct Entity {
    id: u32,
    stats: Stats,
    position: Position,
}

impl Entity {
    fn new(id: u32) -> Self {
        Self {
            id: id,
            stats: Stats {
                hp: 15,
                ap: 10,
                max_ap: 10,
            },
            position: Position { x: 0, y: 0 },
        }
    }
}

struct Stats {
    hp: u32,
    ap: u32,
    max_ap: u32,
}

struct Position {
    x: u32,
    y: u32,
}

enum Action {
    EndTurn,
}

struct GameState {
    rng: StdRng,
    next_entity_id: u32,
    current_turn_id: u32,
    entities: Vec<Entity>,
}

impl GameState {
    fn new(seed: u64) -> Self {
        let mut game_state = Self {
            rng: SeedableRng::seed_from_u64(seed),
            next_entity_id: 1,
            current_turn_id: 1,
            entities: vec![],
        };
        game_state.push_entity();
        game_state.push_entity();

        game_state
    }

    fn get_current_entity(&mut self) -> &mut Entity {
        self.entities
            .iter_mut()
            .find(|x| x.id == self.current_turn_id)
            .unwrap()
    }

    fn push_entity(&mut self) -> u32 {
        let entity_id = self.next_entity_id;
        self.next_entity_id += 1;

        self.entities.push(Entity::new(entity_id));
        entity_id
    }

    fn apply(&mut self, action: Action) -> Result<(), ActionError> {
        match action {
            Action::EndTurn => self.current_turn_id += 1,
        }

        Ok(())
    }

    fn roll_u32(&mut self) -> u32 {
        self.rng.next_u32()
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn generate_random_number_when_seed_is_fixed_should_return_same_number() {
        let mut left = GameState::new(123);
        let mut right = GameState::new(123);

        assert_eq!(left.roll_u32(), right.roll_u32());
    }

    #[test]
    fn generate_random_number_when_seed_is_different_should_not_return_same_number() {
        let mut left = GameState::new(123);
        let mut right = GameState::new(321);

        assert_ne!(left.roll_u32(), right.roll_u32());
    }

    #[test]
    fn push_entity_when_adding_entity_should_increase_next_id_by_one() {
        let mut state = GameState::new(123);
        let before = state.next_entity_id;
        let new_entity_id = state.push_entity();

        assert_eq!(state.next_entity_id, before + 1);
        assert_eq!(new_entity_id, before);
    }

    #[test]
    fn apply_when_current_end_turn_should_swap_next_entity() {
        let mut state = GameState::new(123);
        let before = state.current_turn_id;
        let _ = state.apply(Action::EndTurn);

        let after = state.current_turn_id;
        assert_ne!(before, after);
    }
}
