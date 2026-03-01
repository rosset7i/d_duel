use std::hash::{DefaultHasher, Hash, Hasher};

use rand::{SeedableRng, rngs::StdRng};

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
    Attack(u32),
}

pub struct GameState {
    pub rng: StdRng,
    pub next_entity_id: u32,
    pub current_turn_index: usize,
    pub entities: Vec<Entity>,
}

impl GameState {
    pub fn new(seed: u64) -> Self {
        let mut game_state = Self {
            rng: SeedableRng::seed_from_u64(seed),
            next_entity_id: 1,
            current_turn_index: 0,
            entities: vec![],
        };
        game_state.apply(Action::Spawn);
        game_state.apply(Action::Spawn);

        game_state
    }

    pub fn apply(&mut self, action: Action) {
        match action {
            Action::EndTurn => {
                self.current_turn_index = (self.current_turn_index + 1) % self.entities.len()
            }
            Action::Spawn => {
                let entity_id = self.next_entity_id;
                self.next_entity_id += 1;

                self.entities.push(Entity::new(entity_id));
            }
            Action::Attack(target_id) => {
                let entity = self.get_entity_by_id(target_id).expect("Unvalid target");
                entity.stats.hp -= 3;

                let current = self.get_current().expect("unvalid current entity");
                current.stats.ap -= 2;
            }
        }
    }

    fn get_entity_by_id(&mut self, entity_id: u32) -> Option<&mut Entity> {
        self.entities.iter_mut().find(|x| x.id == entity_id)
    }

    fn get_current(&mut self) -> Option<&mut Entity> {
        self.entities.get_mut(self.current_turn_index)
    }

    pub fn hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.entities.hash(&mut hasher);
        self.current_turn_index.hash(&mut hasher);
        hasher.finish()
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::*;

    #[test]
    fn generate_random_number_when_seed_is_fixed_should_return_same_number() {
        let mut left = GameState::new(123);
        let mut right = GameState::new(123);

        let all: bool = (0..20).all(|_| left.rng.next_u32() == right.rng.next_u32());
        assert!(all);
    }

    #[test]
    fn generate_random_number_when_seed_is_different_should_not_return_same_number() {
        let mut left = GameState::new(123);
        let mut right = GameState::new(321);

        let all: bool = (0..20).all(|_| left.rng.next_u32() == right.rng.next_u32());
        assert!(!all);
    }

    #[test]
    fn push_entity_when_adding_entity_should_increase_next_id_by_one() {
        let mut state = GameState::new(123);
        let before = state.next_entity_id;
        state.apply(Action::Spawn);

        assert_eq!(state.next_entity_id, before + 1);
    }

    #[test]
    fn apply_when_current_end_turn_should_swap_next_entity() {
        let mut state = GameState::new(123);
        let before = state.current_turn_index;
        state.apply(Action::EndTurn);

        let after = state.current_turn_index;
        assert_ne!(before, after);
    }

    #[test]
    fn hash_when_current_state_same_should_be_equal() {
        let mut a = GameState::new(123);
        let mut b = GameState::new(123);

        a.apply(Action::EndTurn);
        b.apply(Action::EndTurn);

        assert_eq!(a.hash(), b.hash());
    }

    #[test]
    fn hash_when_current_state_not_same_should_not_be_equal() {
        let mut a = GameState::new(123);
        let mut b = GameState::new(123);

        a.apply(Action::EndTurn);
        b.apply(Action::Attack(1));

        assert_ne!(a.hash(), b.hash());
    }
}
