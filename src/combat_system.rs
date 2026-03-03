use crate::{entity_manager::Entity, errors::GameError};

const ATTACK_COST: u32 = 4;
const ATTACK_DAMAGE: u32 = 2;
const RANGE: u32 = 2; // 2 For manhattan diagonal attacks

pub fn attack(attacker: &mut Entity, target: &mut Entity) -> Result<(), GameError> {
    if attacker.stats.ap < ATTACK_COST {
        return Err(GameError::NotEnoughActionPoints {
            current: attacker.stats.ap,
            required: ATTACK_COST,
        });
    }

    if attacker
        .position
        .calculate_manhattan_distance(&target.position)
        > RANGE
    {
        return Err(GameError::TargetNotInRange);
    }

    attacker.stats.deduct_ap(ATTACK_COST);
    target.stats.deduct_hp(ATTACK_DAMAGE);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity_manager::Entity;

    #[test]
    fn attack_when_not_enough_ap_should_return_error() {
        let mut attacker = Entity::new(10, 2, 0, 0);
        let mut target = Entity::new(10, 10, 0, 0);

        let result = attack(&mut attacker, &mut target);

        assert!(result.is_err())
    }

    #[test]
    fn attack_when_not_in_range_should_return_error() {
        let mut attacker = Entity::new(10, 4, 0, 0);
        let mut target = Entity::new(10, 10, 10, 10);

        let result = attack(&mut attacker, &mut target);

        assert!(result.is_err())
    }

    #[test]
    fn attack_when_enough_ap_should_return_success() {
        let mut attacker = Entity::new(10, 4, 1, 1);
        let mut target = Entity::new(10, 10, 0, 0);

        let result = attack(&mut attacker, &mut target);

        assert!(result.is_ok())
    }
}
