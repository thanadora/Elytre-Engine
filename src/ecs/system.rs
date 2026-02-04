// system.rs
use crate::ecs::world::World;
use crate::ecs::entity::Entity;

/// Un trait général que tous les systèmes implémentent.
/// Il prend un `&mut World` et fait quelque chose.
pub trait System {
    fn run(&mut self, world: &mut World);
}


impl<F> System for F 
where 
    F: FnMut(&mut World),
    {
        fn run(&mut self, world: &mut World) {
            (self)(world)
        }
    }

pub struct Schedule {
    systems: Vec<Box<dyn System>>,
}

impl Schedule {
    pub fn new() -> Self {
        Self { systems: Vec::new() }
    }

    pub fn add_system<S: System + 'static>(&mut self, system: S) {
        self.systems.push(Box::new(system))
    }

    pub fn run(&mut self, world: &mut World) {
        for system in self.systems.iter_mut() {
            system.run(world);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ecs::world::World; 

    #[derive(Debug, PartialEq,Clone)]
    struct Position { x: f32, y: f32 }
    #[derive(Debug, PartialEq,Clone)]
    struct Velocity { x: f32, y: f32 }

    #[test]
    fn test_basic_system() {
        let mut world = crate::ecs::world::World::new();
        let e1 = world.spawn();
        let e2 = world.spawn();

        world.add_component(e1, Position { x: 0.0, y: 0.0 });
        world.add_component(e1, Velocity { x: 1.0, y: 0.0 });
        world.add_component(e2, Position { x: 5.0, y: 0.0 });
        world.add_component(e2, Velocity { x: 0.0, y: -1.0 });

        let mut schedule = crate::ecs::system::Schedule::new();

        schedule.add_system(|world: &mut crate::ecs::world::World| {
            let positions = world.iter_components::<Position>();
            let entities: Vec<_> = positions.map(|(e, _)| e).collect();


            for entity in entities {
                // on copie Velocity hors du borrow
                if let Some(vel) = world.get_component::<Velocity>(entity).cloned() {
                    if let Some(pos) = world.get_component_mut::<Position>(entity) {
                        pos.x += vel.x;
                        pos.y += vel.y;
                    }
                }
            }


        });

        schedule.run(&mut world);

        assert_eq!(world.get_component::<Position>(e1).unwrap(),
                   &Position { x: 1.0, y: 0.0 });
        assert_eq!(world.get_component::<Position>(e2).unwrap(),
                   &Position { x: 5.0, y: -1.0 });
    }
}
