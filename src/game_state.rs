use crate::{
    entity_manager::{Entity, EntityId, Position},
    errors::GameError,
    map::{GameMap, MAP, in_bounds, is_walkable},
    rng::DeterministicRng,
};
use std::hash::{DefaultHasher, Hash, Hasher};

const RANGE: u32 = 2; // 2 For manhattan diagonal attacks
const ATTACK_DAMAGE: u32 = 2;

pub enum Action {
    Move { actor: EntityId, position: Position },
    Attack { actor: EntityId, target: EntityId },
    Wait { actor: EntityId },
}

impl Action {
    fn actor(&self) -> EntityId {
        match *self {
            Action::Move { actor, .. } => actor,
            Action::Attack { actor, .. } => actor,
            Action::Wait { actor } => actor,
        }
    }
}

enum Event {
    Moved { entity: EntityId, to: Position },
    Damage { target: EntityId, amount: u32 },
    EndTurn,
}

pub struct GameState {
    pub rng: DeterministicRng,
    pub current_actor: EntityId,
    pub entities: Vec<Entity>,
    pub map: GameMap,
    tick: u32,
}

impl GameState {
    pub fn new(seed: u64) -> Self {
        Self {
            rng: DeterministicRng::new(seed),
            current_actor: 1,
            entities: vec![],
            map: MAP,
            tick: 0,
        }
    }

    pub fn step(&mut self, action: Action) -> Result<(), GameError> {
        self.validate(&action)?;
        self.consume_ap(&action)?;
        let events = self.apply_intent(action)?;
        self.resolve(events)?;
        // self.cleanup();
        // self.advance_turn_if_needed();
        self.tick += 1;
        Ok(())
    }

    fn validate(&mut self, action: &Action) -> Result<(), GameError> {
        let actor = action.actor();

        if actor != self.current_actor {
            return Err(GameError::NotYourTurn);
        }

        let entity = self.entity(actor).ok_or(GameError::EntityNotFound(actor))?;
        if entity.is_dead {
            return Err(GameError::ActorDead(actor));
        }

        let cost = entity.action_cost(action);
        if entity.stats.ap < cost {
            return Err(GameError::NotEnoughActionPoints {
                current: entity.stats.ap,
                required: cost,
            });
        }

        match action {
            Action::Move { position, .. } => {
                if !in_bounds(&self.map, &position) {
                    return Err(GameError::OutOfBounds);
                };

                if !is_walkable(&self.map, &position) {
                    return Err(GameError::NotWalkableTile);
                };

                Ok(())
            }
            Action::Attack { target, .. } => {
                let target_entity = self
                    .entity(*target)
                    .ok_or(GameError::EntityNotFound(*target))?;

                if entity
                    .position
                    .calculate_manhattan_distance(&target_entity.position)
                    > RANGE
                {
                    return Err(GameError::TargetNotInRange);
                }

                Ok(())
            }
            Action::Wait { .. } => Ok(()),
        }
    }

    fn consume_ap(&mut self, action: &Action) -> Result<(), GameError> {
        let actor = action.actor();
        let entity = self
            .entity_mut(actor)
            .ok_or(GameError::EntityNotFound(actor))?;

        entity.stats.deduct_ap(entity.action_cost(action));

        Ok(())
    }

    fn apply_intent(&mut self, action: Action) -> Result<Vec<Event>, GameError> {
        match action {
            Action::Move { actor, position } => {
                let events: Vec<Event> = vec![Event::Moved {
                    entity: actor,
                    to: position,
                }];
                Ok(events)
            }
            Action::Attack { target, .. } => {
                let events: Vec<Event> = vec![Event::Damage {
                    target: target,
                    amount: ATTACK_DAMAGE,
                }];
                Ok(events)
            }
            Action::Wait { .. } => Ok(vec![Event::EndTurn]),
        }
    }

    fn resolve(&mut self, events: Vec<Event>) -> Result<(), GameError> {
        for event in events {
            match event {
                Event::Moved { entity, to } => {
                    let entity = self
                        .entity_mut(entity)
                        .ok_or(GameError::EntityNotFound(entity))?;

                    entity.position = to;
                }
                Event::Damage { target, amount } => {
                    let entity = self
                        .entity_mut(target)
                        .ok_or(GameError::EntityNotFound(target))?;

                    entity.stats.deduct_hp(amount);
                }
                Event::EndTurn => (),
            }
        }

        Ok(())
    }

    fn entity(&self, entity_id: EntityId) -> Option<&Entity> {
        self.entities.iter().find(|x| x.id == entity_id)
    }

    fn entity_mut(&mut self, entity_id: EntityId) -> Option<&mut Entity> {
        self.entities.iter_mut().find(|x| x.id == entity_id)
    }

    pub fn hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.entities.hash(&mut hasher);
        self.current_actor.hash(&mut hasher);
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
}
