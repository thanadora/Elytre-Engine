mod ecs;

use ecs::world::World;
use ecs::system::Schedule;
use ecs::entity::Entity;


#[derive(Debug, Clone)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Debug, Clone)]
struct Velocity {
    x: f32,
    y: f32,
}

fn main() {
    let delta_time = 0.016;
    let mut world = World::new();
    let e1 = world.spawn();
    let e2 = world.spawn();

    world.add_component(e1, Position { x: 0.0, y: 0.0 });
    world.add_component(e1, Velocity { x: 1.0, y: 0.0 });
    world.add_component(e2, Position { x: 5.0, y: 0.0 });
    world.add_component(e2, Velocity { x: 0.0, y: -1.0 });

    let mut schedule = Schedule::new();

    schedule.add_system(|world: &mut World| {
        let entities: Vec<Entity> = world
            .iter_components::<Velocity>()
            .map(|(e, _)| e)
            .collect();

        for entity in entities {
            if let Some((pos, vel)) =
                get_storages_mut!(world, entity, Position, Velocity)
            {
                pos.x += vel.x;
                pos.y += vel.y;
            }
        }
    });



    for step in 0..3 {
        println!("--- Step {} ---", step);
        schedule.run(&mut world);

        for (entity, pos) in world.iter_components::<Position>() {
            println!(
                "Entity {:?} -> Position = ({:.1}, {:.1})",
                entity, pos.x, pos.y
            );
        }

        world.increment_time(delta_time);
    }
}
