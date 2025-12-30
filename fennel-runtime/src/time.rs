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
    /// Total elapsed time in seconds
    pub total_elapsed_time: f64,
}

/// System responsible for incrementing ticks
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

impl Tick {
    /// Get the current ticks-per-second rate
    ///
    /// This function divides total ticks amount by total elapsed time, or if the elapsed time is
    /// zero it returns 0.0 to avoid division by zero
    pub fn tps(&self) -> f64 {
        if self.total_elapsed_time > 0.0 {
            self.ticks as f64 / self.total_elapsed_time
        } else {
            0.0
        }
    }
}