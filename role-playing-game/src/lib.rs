// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        match self.health {
            0 => Some(Self {
                health: 100,
                mana: if self.level >= 10 {Some(100)} else {None},
                level: self.level
            }),
            _ => None
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            Some(m) if m < mana_cost => 0,
            Some(ref mut m) => {
                *m -= mana_cost;
                2 * mana_cost
            },
            None if self.health < mana_cost => {
                self.health = 0;
                0
            },
            None => {
                self.health -= mana_cost;
                0
            }
        }
    }
}