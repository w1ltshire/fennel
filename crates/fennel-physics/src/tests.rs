use nalgebra::Vector2;
use crate::body::Body;
use crate::shapes_2d::rigid_body::RigidBody;
use crate::world::PhysicsWorld;

#[test]
fn gravity_free_fall() {
    let mut world = PhysicsWorld::new();
    let mut body = RigidBody::empty();
    body.set_mass(1.0);
    body.set_position(Vector2::new(0.0, 0.0));
    world.add_body(Box::new(body));

    world.step(6.0);

    let position = world.bodies[0].get_position();
    assert_eq!(position.y, -176.58); // calculator told me that a body falling at g=9.8 with v_0 = 0m/s and t=6 will have h=176.58
}