#[derive(Hash)]
pub struct Entity {
    pub id: u32,
    pub stats: Stats,
    pub position: Position,
}

#[derive(Hash)]
pub struct Stats {
    hp: u32,
    pub ap: u32,
    max_ap: u32,
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

#[derive(Hash)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

impl Position {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    pub fn calc_dist(&self, to: &Position) -> u32 {
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
        }
    }

    pub fn set_id(&mut self, id: u32) {
        self.id = id;
    }
}
