use crate::ecs::world::World;
use crate::components::{Transform2D, Velocity2D};
use crate::get_components_mut;



pub fn movement_system(world: &mut World) {
    let transforms = world.query::<Transform2D>().unwrap();
    let entities: Vec<Entity> = transforms.map(|(e, _)| e).collect();

    for entity in entities {
        if let Some((transform, velocity)) = get_components_mut!(world, entity, Transform2D, Velocity2D) {
            transform.position.x += velocity.velocity.x;
            transform.position.y += velocity.velocity.y;
        }
    }
}