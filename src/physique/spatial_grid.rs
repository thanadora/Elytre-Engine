use std::collections::HashMap;
use crate::ecs::world::World;
use crate::ecs::entity::Entity;
use crate::components::{Transform2D, Collider2D};
use crate::get_components_mut;
use crate::math::vec2::Vec2;

pub struct SpatialGrid {
    pub cell_size: f32,
    pub cells: HashMap<(i32, i32), Vec<Entity>>,
}

impl SpatialGrid {
    pub fn new(cell_size: f32) -> Self {
        Self {
            cell_size,
            cells: HashMap::new(),
        }
    }

    fn get_cell(&self, x: f32, y: f32) -> (i32, i32) {
        let cx = (x / self.cell_size).floor() as i32;
        let cy = (y / self.cell_size).floor() as i32;
        (cx, cy)
    }

    pub fn clear(&mut self) {
        self.cells.clear();
    }

    pub fn insert(&mut self, entity: Entity, cell: (i32, i32)) {
        self.cells.entry(cell).or_default().push(entity);
    }

    pub fn insert_in_bounds(&mut self, entity: Entity, min: Vec2, max: Vec2) {
        let (min_cx, min_cy) = self.get_cell(min.x, min.y);
        let (max_cx, max_cy) = self.get_cell(max.x, max.y);

        for cx in min_cx..=max_cx {
            for cy in min_cy..=max_cy {
                self.insert(entity, (cx, cy));
            }
        }
    }
}

/// Système qui remplit la grille à chaque frame
pub fn spatial_hash_system(world: &mut World, grid: &mut SpatialGrid) {
    grid.clear();

    let transforms = world.query::<Transform2D>().unwrap();
    let entities: Vec<_> = transforms.map(|(e, _)| e).collect();

    for entity in entities {
        if let Some((transform, collider)) =
            get_components_mut!(world, entity, Transform2D, Collider2D)
        {
            let pos = transform.position;

            match collider {
                Collider2D::AABB { half_size } => {
                    let min = pos - *half_size;
                    let max = pos + *half_size;

                    grid.insert_in_bounds(entity, min, max);
                }

                Collider2D::Circle { radius } => {
                    let r = *radius;
                    let min = Vec2::new(pos.x - r, pos.y - r);
                    let max = Vec2::new(pos.x + r, pos.y + r);

                    grid.insert_in_bounds(entity, min, max);
                }
            }
        }
    }
}