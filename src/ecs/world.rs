use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::ptr;

use crate::ecs::entity::{Entity, EntityManager};
use crate::ecs::sparse_set::{SparseSet,ComponentStorage};
use crate::ecs::change_time::{ChangeTime, ComponentTimes};


#[macro_export]
macro_rules! get_storages_mut {
    ($world:expr, $entity:expr, $($t:ty),+ $(,)?) => {{
        use std::any::TypeId;

        // 1️⃣ Vérifier l'unicité des types
        let type_ids = &[
            $( TypeId::of::<$t>() ),+
        ];

        for i in 0..type_ids.len() {
            for j in (i + 1)..type_ids.len() {
                assert!(
                    type_ids[i] != type_ids[j],
                    "get_storages_mut!: duplicated component type"
                );
            }
        }

        let time = $world.current_time();
        let world_ptr = $world as *mut crate::ecs::world::World;

        let mut ok = true;

        unsafe {
            // Vérification présence
            $(
                let storage: *mut crate::ecs::sparse_set::SparseSet<$t> =
                    (*world_ptr).get_or_insert_storage::<$t>();

                if (*storage).get($entity).is_none() {
                    ok = false;
                }
            )+

            if !ok {
                None
            } else {
                Some((
                    $(
                        {
                            let storage: *mut crate::ecs::sparse_set::SparseSet<$t> =
                                (*world_ptr).get_or_insert_storage::<$t>();

                            (*storage).get_mut($entity, time).unwrap()
                        }
                    ),+
                ))
            }
        }
    }};
}





pub struct World {
    entities: EntityManager,
    storages: HashMap<TypeId, Box<dyn ComponentStorage>>,
    current_time: ChangeTime
}


impl World {
    
    pub fn new () -> Self {
        Self {
            entities: EntityManager::new(),
            storages: HashMap::new(),
            current_time: ChangeTime::new(),
        }
    }

    //update by frame
    pub fn increment_time(&mut self,delta: f32)
    {
        self.current_time.increment(delta);
    }

    pub fn current_time(&self) -> ChangeTime {
        self.current_time
    }

    pub fn spawn(&mut self) -> Entity {
        self.entities.create()
    }

    pub fn despawn(&mut self, entity: Entity) -> bool {
        if !self.entities.destroy(entity)
        {
            return false;
        }

        for storage in self.storages.values_mut()
        {
            storage.remove_entity(entity);
        }

        true
    }

    pub fn add_component<T: 'static>(&mut self, entity: Entity, component: T)
    {
        let time = self.current_time;
        let storage = self.get_or_insert_storage::<T>();
        storage.insert(entity, component, time);
    } 

    pub fn remove_component<T: 'static>(&mut self, entity: Entity) -> Option<T>
    {
        self.get_storage_mut::<T>().and_then(|s| s.remove(entity))
    }

    pub fn get_component<T: 'static>(&self, entity: Entity) -> Option<&T> {
        self.get_storage::<T>()
            .and_then(|s| s.get(entity))
    }

    pub fn get_component_mut<T: 'static>(&mut self, entity: Entity) -> Option<&mut T> {
        let time = self.current_time;
        self.get_storage_mut::<T>().and_then(|s| s.get_mut(entity,time))
    }

    pub fn iter_components<T: 'static>(&self,) -> impl Iterator<Item = (Entity, &T)> {
        self.get_storage::<T>()
            .into_iter()
            .flat_map(|s| s.iter())
    }

    pub fn is_alive(&self, entity: Entity) -> bool {
        self.entities.is_alive(entity)
    }

    //garenti l existance
    pub(crate) fn get_or_insert_storage<T: 'static>(&mut self) -> &mut SparseSet<T> {
        let type_id = TypeId::of::<T>();
        self.storages
            .entry(type_id)
            .or_insert_with(|| Box::new(SparseSet::<T>::new()) as Box<dyn ComponentStorage>);

        self.storages
            .get_mut(&type_id)
            .unwrap()
            .as_any_mut() 
            .downcast_mut::<SparseSet<T>>()
            .unwrap()
    }

    fn get_storage<T: 'static>(&self) -> Option<&SparseSet<T>> {
        self.storages
            .get(&TypeId::of::<T>())
            .and_then(|s| s.as_any().downcast_ref::<SparseSet<T>>())
    }

    fn get_storage_mut<T: 'static>(&mut self) -> Option<&mut SparseSet<T>> {
        self.storages
            .get_mut(&TypeId::of::<T>())
            .and_then(|s| s.as_any_mut().downcast_mut::<SparseSet<T>>())
    }

    pub fn is_component_changed<T: 'static>(&self, entity: Entity, since: ChangeTime) -> bool {
        self.storages
            .get(&TypeId::of::<T>())
            .and_then(|s| s.as_any().downcast_ref::<SparseSet<T>>())
            .and_then(|s| s.get_times(entity))
            .map(|times| times.is_changed_since(since))
            .unwrap_or(false)
    }

    /// Vérifie si un composant a été ajouté depuis un time donné
    pub fn is_component_added<T: 'static>(&self, entity: Entity, since: ChangeTime) -> bool {
    self.storages
        .get(&TypeId::of::<T>())
        .and_then(|s| s.as_any().downcast_ref::<SparseSet<T>>())
        .and_then(|s| s.get_times(entity))
        .map(|times| times.is_added_since(since))
        .unwrap_or(false)
    }

}