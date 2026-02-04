use crate::ecs::world::World;
use crate::ecs::entity::Entity;
use crate::components::{Transform2D, Collider2D};
use crate::get_components_mut;
use crate::systems::spatial_hash::SpatialHashGrid;

//d^2<(r1​ +r2​)^2
fn circle_vs_circle(a: &Transform2D, ra: f32, b: &Transform2D, rb: f32) -> bool {
    let dx = a.position.x - b.position.x;
    let dy = a.position.y - b.position.y;
    let dist = dx * dx + dy * dy;
    dist < (ra + rb).powi(2)
}

// d<hax​+hbx​
fn aabb_vs_aabb(a: &Transform2D, ha: Vec2, b: &Transform2D, hb: Vec2) -> bool {
    let dx = (a.position.x - b.position.x).abs();
    let dy = (a.position.y - b.position.y).abs();

    dx < (ha.x + hb.x) && dy < (ha.y + hb.y)
}

//trouver le point du rectangle le plus proche du cercle < r 
fn circle_vs_aabb(circle: &Transform2D, radius: f32, rect: &Transform2D, half_size: Vec2) -> bool {
    //distance xy
    let dx = (circle.position.x - rect.position.x).abs();
    let dy = (circle.position.y - rect.position.y).abs();

    //clamp par demi taille pour trouver le point le plus proche 
    let closest_x = dx.min(half_size.x);
    let closest_y = dy.min(half_size.y);


    let dist = (dx - closest_x).powi(2) + (dy - closest_y).powi(2);
    dist < radius.powi(2)
}

pub fn check_collision(a: (&Transform2D, &Collider2D), b: (&Transform2D, &Collider2D)) -> bool {
    match (a.1, b.1) {
        (Circle { radius: ra }, Circle { radius: rb }) =>
            circle_vs_circle(a.0, *ra, b.0, *rb),

        (Circle { radius: r }, AABB { half_size: h }) =>
            circle_vs_aabb(a.0, *r, b.0, *h),

        (AABB { half_size: h }, Circle { radius: r }) =>
            circle_vs_aabb(b.0, *r, a.0, *h),

        (AABB { half_size: ha }, AABB { half_size: hb }) =>
            aabb_vs_aabb(a.0, *ha, b.0, *hb),
}

pub fn collision_system(world: &mut World, grid: &SpatialGrid) {
    let mut checked = std::collections::HashSet::new();

    for ((_cx, _cy), entities) in &grid.cells {
        for i in 0..entities.len() {
            for j in (i + 1)..entities.len() {
                let a = entities[i];
                let b = entities[j];

                // éviter les doublons entre cellules (a,b) = (b,a)
                if !checked.insert((a.min(b), a.max(b))) {
                    continue;
                }

                if let (Some((t1, c1)), Some((t2, c2))) = (
                    get_components_mut!(world, a, Transform2D, Collider2D),
                    get_components_mut!(world, b, Transform2D, Collider2D),
                ) {
                    if check_collision((t1, c1), (t2, c2)) {
                        println!("Collision entre {:?} et {:?}", a, b);
                    }
                }
            }
        }
    }
}


