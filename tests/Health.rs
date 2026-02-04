/*// scripts/health.rs
use crate::scripts::Script;
use crate::game_object::GameObject;
use std::any::Any;

pub struct Health {
    pub max_hp: i32,
    pub current_hp: i32,
}

impl Script for Health {
    fn start(&mut self, _owner: &mut GameObject) {
        println!("Health started: {} HP", self.max_hp);
    }

    fn update(&mut self, _owner: &mut GameObject, _dt: f32) {
        
    }
}
*/