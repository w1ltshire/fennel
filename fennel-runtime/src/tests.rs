use crate::time::Tick;

#[test]
fn tps_calculation() {
    let tick = Tick {
        ticks: 500,
        tick_rate: 100_000_000,
        total_elapsed_time: 5.0,
    };

    assert_eq!(tick.tps(), 100.0);
}