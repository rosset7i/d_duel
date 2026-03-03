use crate::{
    combat_system::attack,
    entity_manager::{Entity, Position},
    errors::GameError,
    map::{GameMap, MAP, in_bounds, is_walkable},
    rng::DeterministicRng,
};
use std::hash::{DefaultHasher, Hash, Hasher};

pub enum Action {
    Spawn(Entity),
    EndTurn,
    Move { entity_id: u32, position: Position },
    Attack { attacker_id: u32, target_id: u32 },
}

pub struct GameState {
    pub rng: DeterministicRng,
    pub next_entity_id: u32,
    pub current_turn_index: usize,
    pub entities: Vec<Entity>,
    pub map: GameMap,
}

impl GameState {
    pub fn new(seed: u64) -> Self {
        Self {
            rng: DeterministicRng::new(seed),
            next_entity_id: 1,
            current_turn_index: 0,
            entities: vec![],
            map: MAP,
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
            Action::Attack {
                attacker_id,
                target_id,
            } => {
                let attacker_index = self
                    .get_index_by_id(attacker_id)
                    .ok_or(GameError::EntityNotFound(attacker_id))?;

                let target_index = self
                    .get_index_by_id(target_id)
                    .ok_or(GameError::EntityNotFound(target_id))?;

                let [attacker, target] = self
                    .entities
                    .get_disjoint_mut([attacker_index, target_index])
                    .map_err(|_| GameError::NoEntities)?;

                attack(attacker, target)?;

                Ok(())
            }
            Action::Move {
                entity_id,
                position,
            } => {
                if !in_bounds(&self.map, &position) {
                    return Err(GameError::OutOfBounds);
                };

                if !is_walkable(&self.map, &position) {
                    return Err(GameError::NotWalkableTile);
                };

                let index = self
                    .get_index_by_id(entity_id)
                    .ok_or(GameError::EntityNotFound(entity_id))?;

                let entity = self
                    .entities
                    .get_mut(index)
                    .ok_or(GameError::EntityNotFound(entity_id))?;

                let ap_necessary = entity.position.calculate_manhattan_distance(&position);

                // For now, each tile will be 1 AP
                if entity.stats.ap < ap_necessary {
                    return Err(GameError::NotEnoughActionPoints {
                        current: entity.stats.ap,
                        required: ap_necessary,
                    });
                }

                entity.position = position;
                entity.stats.deduct_ap(ap_necessary);

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
    fn rng_produces_same_sequence_when_seed_is_same() {
        let mut left = GameState::new(123);
        let mut right = GameState::new(123);

        let all: bool = (0..20).all(|_| left.rng.roll() == right.rng.roll());
        assert!(all);
    }

    #[test]
    fn rng_produces_different_sequence_when_seeds_are_different() {
        let mut left = GameState::new(123);
        let mut right = GameState::new(321);

        let all: bool = (0..20).all(|_| left.rng.roll() == right.rng.roll());
        assert!(!all);
    }

    #[test]
    fn spawn_increases_next_entity_id_when_executed() {
        let mut state = GameState::new(123);
        let before = state.next_entity_id;

        state.apply(Action::Spawn(Entity::new(1, 1, 1, 1))).unwrap();

        assert_eq!(state.next_entity_id, before + 1);
    }

    #[test]
    fn spawn_should_return_error_when_entity_has_id() {
        let mut game_state = get_game_state_with_entities(123);
        let mut entity = Entity::new(1, 4, 1, 1);
        entity.set_id(1);

        let result = game_state.apply(Action::Spawn(entity));

        assert!(result.is_err());
    }

    #[test]
    fn end_turn_changes_turn_index_when_executed() {
        let mut state = get_game_state_with_entities(123);
        let before = state.current_turn_index;

        state.apply(Action::EndTurn).unwrap();

        let after = state.current_turn_index;
        assert_ne!(before, after);
    }

    #[test]
    fn end_turn_returns_error_when_no_entities() {
        let mut state = GameState::new(123);

        let result = state.apply(Action::EndTurn);

        assert!(result.is_err());
    }

    #[test]
    fn attack_returns_error_when_target_not_found() {
        let mut state = get_game_state_with_entities(123);

        let result = state.apply(Action::Attack {
            attacker_id: 1,
            target_id: 123,
        });

        assert!(result.is_err());
    }

    #[test]
    fn hash_should_be_equal_when_state_is_same() {
        let mut a = get_game_state_with_entities(123);
        let mut b = get_game_state_with_entities(123);

        a.apply(Action::EndTurn).unwrap();
        b.apply(Action::EndTurn).unwrap();

        assert_eq!(a.hash(), b.hash());
    }

    #[test]
    fn hash_should_differ_when_state_diverges() {
        let mut a = get_game_state_with_entities(123);
        let mut b = get_game_state_with_entities(123);

        a.apply(Action::EndTurn).unwrap();
        b.apply(Action::Attack {
            attacker_id: 1,
            target_id: 2,
        })
        .unwrap();

        assert_ne!(a.hash(), b.hash());
    }

    #[test]
    fn move_returns_error_when_entity_not_found() {
        let mut state = get_game_state_with_entities(123);

        let result = state.apply(Action::Move {
            entity_id: 99,
            position: Position::new(0, 0),
        });

        assert!(result.is_err());
    }

    #[test]
    fn move_returns_error_when_not_enough_ap() {
        let mut state = get_game_state_with_entities(123);

        let result = state.apply(Action::Move {
            entity_id: 1,
            position: Position::new(5, 5),
        });

        assert!(result.is_err());
    }

    #[test]
    fn move_returns_error_when_out_of_bounds() {
        let mut game_state = GameState::new(123);
        game_state
            .apply(Action::Spawn(Entity::new(1, 100, 1, 1)))
            .unwrap();

        let result = game_state.apply(Action::Move {
            entity_id: 1,
            position: Position::new(10, 10),
        });

        assert!(result.is_err());
    }

    #[test]
    fn move_returns_error_when_not_walkable() {
        let mut game_state = GameState::new(123);
        game_state
            .apply(Action::Spawn(Entity::new(1, 100, 1, 1)))
            .unwrap();

        let result = game_state.apply(Action::Move {
            entity_id: 1,
            position: Position::new(0, 0),
        });

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
