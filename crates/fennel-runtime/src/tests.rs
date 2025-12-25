use crate::camera::Camera;
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

#[test]
fn camera() {
    let camera = Camera::new(
        (100.0, 100.0),
        (0.0, 0.0)
    );

    let camera_pos = camera.world_to_camera((50.0, 50.0));
    assert_eq!(camera_pos, (-50.0, -50.0));
}