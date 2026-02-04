use std::any::Any;
use log::warn;


use crate::ecs::entity::Entity;
use crate::ecs::change_time::{ComponentTimes, ChangeTime};

const EMPTY: u32 = u32::MAX;

#[derive(Debug)]
pub struct SparseSet<T> {
    sparse: Vec<u32>,    // sparse[id] -> index dans dense
    dense: Vec<Entity>,  // dense[index] -> Entity
    data: Vec<T>,        // data[index] -> T
    times: Vec<ComponentTimes> // Métadonnées de changement
}

pub trait ComponentStorage {
    fn remove_entity(&mut self, entity: Entity);

    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}


impl<T: 'static> ComponentStorage for SparseSet<T> {    
    fn remove_entity(&mut self, entity: Entity) {
        let _ = self.remove(entity);
    }
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any { 
        self
    }
}

impl<T> SparseSet<T> {
    pub fn new() -> Self {
        Self {
            sparse: Vec::new(),
            dense: Vec::new(),
            data: Vec::new(),
            times: Vec::new(),
        }
    }

    pub fn insert(&mut self, entity: Entity, value: T, current_time: ChangeTime)
    {
        let id = entity.id as usize;
        if id >= self.sparse.len() {
            self.sparse.resize(id + 1, EMPTY);
        }

        let dense_index = self.sparse[id];

        if dense_index == EMPTY {
            let new_index = self.dense.len() as u32;
            self.sparse[id] = new_index;
            self.dense.push(entity);
            self.data.push(value);
            self.times.push(ComponentTimes::new(current_time));
        }
        else {
            self.data[dense_index  as usize] = value;
            self.times[dense_index as usize].set_changed(current_time);
        }
    }

    pub fn remove(&mut self, entity: Entity) -> Option<T> {
        let id = entity.id as usize;
        if id >= self.sparse.len() {
            warn!("delete something not existe");
            return None;
        }

         let dense_index = self.sparse[id];
        if dense_index == EMPTY {
            warn!("delete something not existe");
            return None;
        }

        let dense_index = dense_index as usize;
        if self.dense[dense_index].generation != entity.generation {
            warn!("delete something not existe");
            return None;
        }

        let removed_value = self.data.swap_remove(dense_index);
        self.dense.swap_remove(dense_index);
        self.times.swap_remove(dense_index);


        if dense_index < self.dense.len() {
            let moved_entity = self.dense[dense_index];
            self.sparse[moved_entity.id as usize] = dense_index as u32;
        }

        self.sparse[id] = EMPTY;

        Some(removed_value)
        
    }

    pub fn get(&self, entity: Entity) -> Option<&T> {
        let id = entity.id as usize;
        if id >= self.sparse.len() {
            return None;
        }

        let dense_index = self.sparse[id];
        if dense_index == EMPTY {
            return None;
        }

        let dense_index = dense_index as usize;
        if self.dense[dense_index].generation  != entity.generation  {
            return None;
        }

        self.data.get(dense_index)
    }

    pub fn get_mut(&mut self, entity: Entity, current_time: ChangeTime) -> Option<&mut T> {
        let id = entity.id as usize;
        if id >= self.sparse.len() {
            return None;
        }

        let dense_index = self.sparse[id];
        if dense_index == EMPTY {
            return None;
        }

        let dense_index = dense_index as usize;
        if self.dense[dense_index].generation  != entity.generation  {
            return None;
        }

        self.times[dense_index].set_changed(current_time);
        Some(&mut self.data[dense_index])

    }

    pub fn contains(&self, entity: Entity) -> bool {
        
        let id = entity.id as usize;
        if id >= self.sparse.len() {
            return false;
        }

        let dense_index = self.sparse[id];
        if dense_index == EMPTY {
            return false;
        }

        let dense_index = dense_index as usize;
        self.dense[dense_index] == entity
    }

    pub fn iter(&self) -> impl Iterator<Item = (Entity, &T)> {
        self.dense.iter().copied().zip(self.data.iter())
    }

    pub fn iter_mut(&mut self, current_time: ChangeTime) -> impl Iterator<Item = (Entity, &mut T)> {
        let dense = &self.dense;
        let times = &mut self.times;
        let data = &mut self.data;

        dense
            .iter()
            .copied()
            .zip(times.iter_mut())
            .zip(data.iter_mut())
            .map(move |((entity, time), value)| {
                time.set_changed(current_time);
                (entity, value)
            })
    }


    
    pub fn get_times(&self, entity: Entity) -> Option<ComponentTimes> {
        let id = entity.id as usize;
        if id >= self.sparse.len() {
            return None;
        }

        let dense_index = self.sparse[id];
        if dense_index == EMPTY {
            return None;
        }
        let dense_index = dense_index as usize;
        if self.dense[dense_index].generation != entity.generation {
            return None;
        }
        self.times.get(dense_index).copied()
    }

}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::ecs::entity::Entity;
    use crate::ecs::change_time::{ChangeTime, ComponentTimes};

    fn make_entity(id: u32, generation: u32) -> Entity {
        Entity { id, generation }
    }

    #[test]
    fn insert_and_get() {
        let mut set = SparseSet::<i32>::new();
        let e = make_entity(1, 0);
        let time = ChangeTime::new();

        set.insert(e, 42, time);
        assert_eq!(set.get(e), Some(&42));
        assert!(set.contains(e));
    }

    #[test]
    fn insert_updates_existing() {
        let mut set = SparseSet::<i32>::new();
        let e = make_entity(2, 0);
        let time = ChangeTime::new();

        set.insert(e, 10, time);
        set.insert(e, 99, time);
        assert_eq!(set.get(e), Some(&99));
    }

    #[test]
    fn get_mut_updates_time() {
        let mut set = SparseSet::<i32>::new();
        let e = make_entity(3, 0);
        let time = ChangeTime::new();

        set.insert(e, 5, time);
        let new_time = ChangeTime(1.0); 
        if let Some(val) = set.get_mut(e, new_time) {
            *val = 77;
        }
        assert_eq!(set.get(e), Some(&77));

        // Vérifie que le time a été mis à jour
        let times = set.get_times(e).unwrap();
        assert!(times.is_changed_since(time));
    }

    #[test]
    fn remove_entity() {
        let mut set = SparseSet::<i32>::new();
        let e = make_entity(4, 0);
        let time = ChangeTime::new();

        set.insert(e, 123, time);
        let removed = set.remove(e);
        assert_eq!(removed, Some(123));
        assert!(!set.contains(e));
        assert_eq!(set.get(e), None);
    }

    #[test]
    fn remove_nonexistent_returns_none() {
        let mut set = SparseSet::<i32>::new();
        let e = make_entity(5, 0);
        assert_eq!(set.remove(e), None);
    }

    #[test]
    fn iter_and_iter_mut() {
        let mut set = SparseSet::<i32>::new();
        let time = ChangeTime::new();

        let e1 = make_entity(10, 0);
        let e2 = make_entity(11, 0);

        set.insert(e1, 1, time);
        set.insert(e2, 2, time);

        // Test iter
        let items: Vec<_> = set.iter().collect();
        assert_eq!(items.len(), 2);
        assert!(items.contains(&(e1, &1)));
        assert!(items.contains(&(e2, &2)));

        // Test iter_mut
        for (_, val) in set.iter_mut(ChangeTime::new()) {
            *val += 10;
        }
        assert_eq!(set.get(e1), Some(&11));
        assert_eq!(set.get(e2), Some(&12));
    }

    #[test]
    fn get_times_none_for_nonexistent() {
        let set = SparseSet::<i32>::new();
        let e = make_entity(99, 0);
        assert_eq!(set.get_times(e), None);
    }
}
