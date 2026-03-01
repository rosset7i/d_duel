use std::hash::{DefaultHasher, Hash, Hasher};

use rand_chacha::{
    ChaCha8Rng,
    rand_core::{Rng, SeedableRng},
};

#[derive(Debug)]
pub enum GameError {
    EntityNotFound,
    NoEntities,
}

#[derive(Hash)]
pub struct Entity {
    id: u32,
    stats: Stats,
    position: Position,
}

impl Entity {
    fn new(id: u32) -> Self {
        Self {
            id,
            stats: Stats {
                hp: 15,
                ap: 10,
                max_ap: 10,
            },
            position: Position { x: 0, y: 0 },
        }
    }
}

#[derive(Hash)]
struct Stats {
    hp: u32,
    ap: u32,
    max_ap: u32,
}

#[derive(Hash)]
struct Position {
    x: u32,
    y: u32,
}

pub enum Action {
    Spawn,
    EndTurn,
    Attack { attacker: u32, target: u32 },
}

pub struct DeterministicRng {
    rng: ChaCha8Rng,
    seed: u64,
    calls: u8,
}

impl DeterministicRng {
    pub fn roll(&mut self) -> u32 {
        self.calls += 1;
        self.rng.next_u32()
    }
}

pub struct GameState {
    pub rng: DeterministicRng,
    pub next_entity_id: u32,
    pub current_turn_index: usize,
    pub entities: Vec<Entity>,
}

impl GameState {
    pub fn new(seed: u64) -> Self {
        Self {
            rng: DeterministicRng {
                rng: ChaCha8Rng::seed_from_u64(seed),
                seed,
                calls: 0,
            },
            next_entity_id: 1,
            current_turn_index: 0,
            entities: vec![],
        }
    }

    pub fn apply(&mut self, action: Action) -> Result<(), GameError> {
        match action {
            Action::EndTurn => {
                if self.entities.is_empty() {
                    return Err(GameError::NoEntities);
                }

                self.current_turn_index = (self.current_turn_index + 1) % self.entities.len();
                Ok(())
            }
            Action::Spawn => {
                let entity_id = self.next_entity_id;
                self.next_entity_id += 1;

                self.entities.push(Entity::new(entity_id));
                Ok(())
            }
            Action::Attack { attacker, target } => {
                let entity = self
                    .get_entity_by_id(target)
                    .ok_or(GameError::EntityNotFound)?;
                entity.stats.hp -= 3;

                let current = self
                    .get_entity_by_id(attacker)
                    .ok_or(GameError::EntityNotFound)?;
                current.stats.ap -= 2;

                Ok(())
            }
        }
    }

    fn get_entity_by_id(&mut self, entity_id: u32) -> Option<&mut Entity> {
        let index = self.entities.iter().position(|x| x.id == entity_id)?;
        self.entities.get_mut(index)
    }

    pub fn hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.entities.hash(&mut hasher);
        self.current_turn_index.hash(&mut hasher);
        self.next_entity_id.hash(&mut hasher);
        self.rng.seed.hash(&mut hasher);
        self.rng.calls.hash(&mut hasher);
        hasher.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_random_number_when_seed_is_fixed_should_return_same_number() {
        let mut left = GameState::new(123);
        let mut right = GameState::new(123);

        let all: bool = (0..20).all(|_| left.rng.roll() == right.rng.roll());
        assert!(all);
    }

    #[test]
    fn generate_random_number_when_seed_is_different_should_not_return_same_number() {
        let mut left = GameState::new(123);
        let mut right = GameState::new(321);

        let all: bool = (0..20).all(|_| left.rng.roll() == right.rng.roll());
        assert!(!all);
    }

    #[test]
    fn spawn_when_adding_entity_should_increase_next_id_by_one() {
        let mut state = GameState::new(123);
        let before = state.next_entity_id;
        state.apply(Action::Spawn).unwrap();

        assert_eq!(state.next_entity_id, before + 1);
    }

    #[test]
    fn apply_when_current_end_turn_should_swap_next_entity() {
        let mut state = get_game_state_with_entities(123);
        let before = state.current_turn_index;
        state.apply(Action::EndTurn).unwrap();

        let after = state.current_turn_index;
        assert_ne!(before, after);
    }

    #[test]
    fn end_turn_when_no_entities_should_return_error() {
        let mut state = GameState::new(123);

        let result = state.apply(Action::EndTurn);

        assert!(result.is_err());
    }

    #[test]
    fn attack_when_target_invalid_should_return_error() {
        let mut state = get_game_state_with_entities(123);

        let result = state.apply(Action::Attack {
            attacker: 1,
            target: 123,
        });

        assert!(result.is_err());
    }

    #[test]
    fn hash_when_current_state_same_should_be_equal() {
        let mut a = get_game_state_with_entities(123);
        let mut b = get_game_state_with_entities(123);

        a.apply(Action::EndTurn).unwrap();
        b.apply(Action::EndTurn).unwrap();

        assert_eq!(a.hash(), b.hash());
    }

    #[test]
    fn hash_when_current_state_not_same_should_not_be_equal() {
        let mut a = get_game_state_with_entities(123);
        let mut b = get_game_state_with_entities(123);

        a.apply(Action::EndTurn).unwrap();
        b.apply(Action::Attack {
            attacker: 1,
            target: 2,
        })
        .unwrap();

        assert_ne!(a.hash(), b.hash());
    }

    fn get_game_state_with_entities(seed: u64) -> GameState {
        let mut game_state = GameState::new(seed);

        game_state.apply(Action::Spawn).unwrap();
        game_state.apply(Action::Spawn).unwrap();

        game_state
    }
}
