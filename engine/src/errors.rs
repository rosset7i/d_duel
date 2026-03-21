use crate::entity_manager::{EntityId, Position};

#[derive(Debug)]
pub enum GameError {
    NotYourTurn(EntityId),
    EntityNotFound(EntityId),
    ActorDead(EntityId),
    NotEnoughActionPoints { current: u32, required: u32 },
    OutOfBounds(Position),
    NotWalkableTile(Position),
    TileOccupied(Position),
    TargetNotInRange(EntityId),
    TargetDead(EntityId),
    NoAvailableActor,
}
