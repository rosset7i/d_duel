use crate::{
    combat_system::attack, entity_manager::Entity, errors::GameError, rng::DeterministicRng,
};
use std::hash::{DefaultHasher, Hash, Hasher};

pub enum Action {
    Spawn(Entity),
    EndTurn,
    Attack { attacker: u32, target: u32 },
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
            rng: DeterministicRng::new(seed),
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
            Action::Spawn(mut entity) => {
                if entity.id != 0 {
                    return Err(GameError::CannotSpawnEntityWithSameId(entity.id));
                }

                let entity_id = self.next_entity_id;

                entity.set_id(entity_id);
                self.entities.push(entity);

                self.next_entity_id += 1;
                Ok(())
            }
            Action::Attack { attacker, target } => {
                let attacker_index = self
                    .get_index_by_id(attacker)
                    .ok_or(GameError::EntityNotFound(attacker))?;

                let target_index = self
                    .get_index_by_id(target)
                    .ok_or(GameError::EntityNotFound(target))?;

                let [attacker, target] = self
                    .entities
                    .get_disjoint_mut([attacker_index, target_index])
                    .map_err(|_| GameError::NoEntities)?;

                attack(attacker, target)?;

                Ok(())
            }
        }
    }

    fn get_index_by_id(&mut self, entity_id: u32) -> Option<usize> {
        self.entities.iter().position(|x| x.id == entity_id)
    }

    pub fn hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.entities.hash(&mut hasher);
        self.current_turn_index.hash(&mut hasher);
        self.next_entity_id.hash(&mut hasher);
        self.rng.hash(&mut hasher);
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
        state.apply(Action::Spawn(Entity::new(1, 1, 1, 1))).unwrap();

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

    #[test]
    fn spawn_when_try_to_spawn_with_same_id_should_return_error() {
        let mut game_state = get_game_state_with_entities(123);
        let mut entity = Entity::new(1, 4, 1, 1);
        entity.set_id(1);

        let result = game_state.apply(Action::Spawn(entity));

        assert!(result.is_err());
    }

    fn get_game_state_with_entities(seed: u64) -> GameState {
        let mut game_state = GameState::new(seed);

        game_state
            .apply(Action::Spawn(Entity::new(1, 4, 1, 1)))
            .unwrap();
        game_state
            .apply(Action::Spawn(Entity::new(1, 1, 1, 1)))
            .unwrap();

        game_state
    }
}
