use crate::game_state::Action;

pub type EntityId = u32;

const ATTACK_COST: u32 = 4;

#[derive(Hash)]
pub struct Entity {
    pub id: EntityId,
    pub stats: Stats,
    pub position: Position,
    pub is_dead: bool,
}

#[derive(Hash)]
pub struct Stats {
    pub hp: u32,
    pub ap: u32,
    pub max_ap: u32,
}

impl Stats {
    pub fn new(hp: u32, ap: u32) -> Self {
        Self { hp, ap, max_ap: ap }
    }

    pub fn deduct_ap(&mut self, cost: u32) {
        let (remainder, overflowed) = self.ap.overflowing_sub(cost);

        if overflowed {
            self.ap = 0;
        } else {
            self.ap = remainder;
        };
    }

    pub fn deduct_hp(&mut self, damage: u32) {
        let (remainder, overflowed) = self.hp.overflowing_sub(damage);

        if overflowed {
            self.hp = 0;
        } else {
            self.hp = remainder;
        };
    }
}

#[derive(Hash, PartialEq, Debug)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

impl Position {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    pub fn calculate_manhattan_distance(&self, to: &Position) -> u32 {
        let x = self.x.abs_diff(to.x);
        let y = self.y.abs_diff(to.y);

        x + y
    }
}

impl Entity {
    pub fn new(hp: u32, ap: u32, pos_x: u32, pos_y: u32) -> Self {
        Self {
            id: 0,
            stats: Stats::new(hp, ap),
            position: Position::new(pos_x, pos_y),
            is_dead: false,
        }
    }

    pub fn action_cost(&self, action: &Action) -> u32 {
        match action {
            Action::Move { position, .. } => self.position.calculate_manhattan_distance(&position),
            Action::Attack { .. } => ATTACK_COST,
            Action::Wait { .. } => self.stats.ap,
        }
    }

    pub fn set_id(&mut self, id: u32) {
        self.id = id;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deduct_ap_should_return_zero_when_overflow() {
        let mut stats = Stats::new(1, 4);

        stats.deduct_ap(5);

        assert_eq!(stats.ap, 0);
    }

    #[test]
    fn deduct_hp_should_return_zero_when_overflow() {
        let mut stats = Stats::new(4, 1);

        stats.deduct_hp(5);

        assert_eq!(stats.hp, 0);
    }
}
