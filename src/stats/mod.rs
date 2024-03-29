use std::cmp::{max, min};

use bevy::prelude::*;

pub trait Stat {
    fn damage(&mut self, amount: usize);
    fn restore(&mut self, amount: usize);
    fn current(&self) -> usize;
    fn max(&self) -> usize;
}

#[derive(Component)]
pub struct Health {
    current: usize,
    max: usize,
}

impl Health {
    pub fn new(max: usize) -> Self {
        Self { current: max, max }
    }

    pub fn new_at(current: usize, max: usize) -> Self {
        Self { current, max }
    }
}

impl Stat for Health {
    fn damage(&mut self, amount: usize) {
        self.current = max(0, self.current - amount);
    }

    fn restore(&mut self, amount: usize) {
        self.current = min(self.max, self.current + amount);
    }

    fn current(&self) -> usize {
        self.current
    }

    fn max(&self) -> usize {
        self.max
    }
}
