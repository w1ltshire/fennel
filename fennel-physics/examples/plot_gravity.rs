use plotters::prelude::*;
use fennel_physics::body::Body;
use fennel_physics::shapes_2d::rigid_body::RigidBody;
use fennel_physics::world::PhysicsWorld;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut world = PhysicsWorld::new();
    let mut body = RigidBody::empty();
    body.set_mass(1.0);
    body.set_position(nalgebra::Vector2::new(0.0, 100.0));
    world.add_body(Box::new(body));

    let dt = 1.0 / 60.0_f32;
    let steps = 600;
    let mut samples: Vec<(f32, f32)> = Vec::with_capacity(steps + 1);

    let t0 = 0.0_f32;
    let y0 = world.bodies[0].get_position().y;
    samples.push((t0, y0));

    for i in 1..=steps {
        world.step(dt);
        let t = i as f32 * dt;
        let y = world.bodies[0].get_position().y;
        println!("step: {}, y: {}", i, y);
        samples.push((t, y));
    }

    let out_path = "free_fall.png";
    let root = BitMapBackend::new(out_path, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let t_min = samples.first().unwrap().0;
    let t_max = samples.last().unwrap().0;
    let y_min = samples.iter().map(|s| s.1).fold(f32::INFINITY, f32::min);
    let y_max = samples.iter().map(|s| s.1).fold(f32::NEG_INFINITY, f32::max);

    let mut chart = ChartBuilder::on(&root)
        .caption("Free Fall", ("sans-serif", 30).into_font())
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(t_min..t_max, (y_min - 1.0)..(y_max + 1.0))?;

    chart.configure_mesh().x_desc("time (s)").y_desc("y (m)").draw()?;

    chart
        .draw_series(LineSeries::new(
            samples.into_iter(),
            &BLUE,
        ))?
        .label("y(t)")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLUE));

    chart.configure_series_labels().background_style(WHITE.mix(0.8)).border_style(BLACK).draw()?;

    root.present()?;
    println!("Wrote {}", out_path);

    Ok(())
}