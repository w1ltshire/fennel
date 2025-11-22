use std::ops::AddAssign;
use specs::{System, VecStorage, WriteExpect};
use specs::Component;

/// A struct representing game ticks
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Tick {
    /// Amount of ticks ran since the game start
    pub ticks: u64,
    /// The tick rate in nanoseconds per tick
    pub tick_rate: u64,
}

pub struct TickSystem;

impl<'a> System<'a> for TickSystem {
    type SystemData = WriteExpect<'a, Tick>;

    fn run(&mut self, mut ticks: Self::SystemData) {
        *ticks += 1;
    }
}

impl AddAssign<u64> for Tick {
    fn add_assign(&mut self, rhs: u64) {
        self.ticks += rhs;
    }
}