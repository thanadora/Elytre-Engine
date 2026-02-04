use log::{error, warn, info, debug};


#[derive(Copy,Clone,Debug,Eq,PartialEq,Hash)]
pub struct Entity {
    pub id: u32,
    pub generation: u32,
}



pub struct EntityManager{
    generations: Vec<u32>,
    free: Vec<u32>,
}


impl EntityManager {
    pub fn new() -> Self 
    {
        Self {
            generations: Vec::new(),
            free: Vec::new(),
        }
    }

    pub fn create(&mut self) -> Entity {
    
        let id;

        let id_opt = self.free.pop();
        
        if let Some(val) = id_opt 
        {
            id = val;
        } 
        else 
        {
            id = self.generations.len() as u32;
            self.generations.push(0);
        }
        
        let generation = self.generations[id as usize];
        Entity {id, generation}
    }
    
    pub fn destroy(&mut self, entity: Entity) -> bool {
        let id = entity.id as usize;
        if id >= self.generations.len()
        {
            error!("bad id");
            return false; 
        }

        if self.generations[id] != entity.generation
        {
            warn!("bad generation");
            return false;
        }

        self.generations[id] = self.generations[id].checked_add(1).expect("Generation overflow: too many entities created!");
        self.free.push(entity.id);
        true
    }

    pub fn is_alive(&self, entity: Entity) -> bool {
        let id = entity.id as usize;
        if id < self.generations.len() {
            self.generations[id] == entity.generation
        } 
        else {
            false
        }
    }


}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_entity() {
        let mut manager = EntityManager::new();
        let e1 = manager.create();
        assert_eq!(e1.id, 0);
        assert_eq!(e1.generation, 0);

        let e2 = manager.create();
        assert_eq!(e2.id, 1);
        assert_eq!(e2.generation, 0);
    }

    #[test]
    fn test_destroy_and_reuse_id() {
        let mut manager = EntityManager::new();
        let e1 = manager.create();
        assert!(manager.destroy(e1));

        //use free id
        let e2 = manager.create();
        assert_eq!(e2.id, e1.id);
        assert_eq!(e2.generation, e1.generation + 1);
    }

    #[test]
    fn test_destroy_invalid_id() {
        let mut manager = EntityManager::new();
        let e1 = Entity { id: 42, generation: 0 };
        assert!(!manager.destroy(e1)); // id too big
    }

    #[test]
    fn test_destroy_wrong_generation() {
        let mut manager = EntityManager::new();
        let e1 = manager.create();
        let bad_entity = Entity { id: e1.id, generation: e1.generation + 1 };
        assert!(!manager.destroy(bad_entity)); // log "bad generation"
    }


    #[test]
    fn test_is_alive_true_for_created_entity() {
        let mut manager = EntityManager::new();
        let e1 = manager.create();
        assert!(manager.is_alive(e1));
    }

    #[test]
    fn test_is_alive_false_for_invalid_id() {
        let manager = EntityManager::new();
        let fake = Entity { id: 42, generation: 0 };
        assert!(!manager.is_alive(fake));
    }

    #[test]
    fn test_is_alive_false_after_destroy() {
        let mut manager = EntityManager::new();
        let e1 = manager.create();
        assert!(manager.is_alive(e1));

        manager.destroy(e1);
        assert!(!manager.is_alive(e1));
    }

    #[test]
    fn test_is_alive_true_for_recreated_entity() {
        let mut manager = EntityManager::new();
        let e1 = manager.create();
        manager.destroy(e1);

        let e2 = manager.create();

        assert!(manager.is_alive(e2));
        assert!(!manager.is_alive(e1)); 
    }

    #[test]
    fn test_double_destroy_fails() {
        let mut manager = EntityManager::new();
        let e = manager.create();
        assert!(manager.destroy(e));
        assert!(!manager.destroy(e));
    }

    #[test]
    fn test_destroy_old_entity_after_reuse() {
        let mut manager = EntityManager::new();
        let e1 = manager.create();
        manager.destroy(e1);
        let _e2 = manager.create();

        assert!(!manager.destroy(e1));
    }


}
