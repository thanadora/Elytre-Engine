pub fn gravity_system(world: &mut World) {
    let entities: Vec<_> = world.query::<Velocity2D>().unwrap().map(|(e, _)| e).collect();
    for entity in entities {
        if let Some(vel) = world.get_component_mut::<Velocity2D>(entity) {
            vel.velocity.y -= 0.1; 
        }
    }
}
