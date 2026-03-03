#[derive(Debug)]
pub enum GameError {
    EntityNotFound(u32),
    NoEntities,
    TargetNotInRange,
    CannotSpawnEntityWithSameId(u32),
    NotEnoughActionPoints { current: u32, required: u32 },
}
