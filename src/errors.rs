use crate::entity_manager::EntityId;

#[derive(Debug)]
pub enum GameError {
    NotYourTurn,
    EntityNotFound(EntityId),
    ActorDead(EntityId),
    NoEntities,
    TargetNotInRange,
    CannotSpawnEntityWithSameId(EntityId),
    NotEnoughActionPoints { current: u32, required: u32 },
    OutOfBounds,
    NotWalkableTile,
}
