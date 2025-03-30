use std::any::{Any, TypeId};
use std::collections::HashMap;

use super::core::{Handle, Resource, ResourceId};

// Actually stores the resources in a HashMap
struct ResourceStorage<T: Resource> {
    resources: HashMap<ResourceId, T>,
    parameters_cache: HashMap<T::Parameters, ResourceId>,
}

impl<T: Resource> ResourceStorage<T> {
    fn new() -> Self {
        Self {
            resources: HashMap::new(),
            parameters_cache: HashMap::new(),
        }
    }

    fn insert_resource(&mut self, id: ResourceId, resource: T, parameters: T::Parameters) {
        self.resources.insert(id, resource);
        self.parameters_cache.insert(parameters, id);
    }
}

pub struct ResourceManager {
    storages: HashMap<TypeId, Box<dyn Any>>,
    next_id: ResourceId,
}

impl ResourceManager {
    pub fn new() -> Self {
        Self {
            storages: HashMap::new(),
            next_id: 0,
        }
    }

    fn storage<T: Resource>(&mut self) -> &mut ResourceStorage<T> {
        let type_id = TypeId::of::<T>();
        self.storages
            .entry(type_id)
            .or_insert(Box::new(ResourceStorage::<T>::new()))
            .downcast_mut()
            .unwrap()
    }

    pub fn load_resource<T: Resource>(&mut self, parameters: &T::Parameters) -> Option<Handle<T>> {
        if let Some(id) = self.find_cached_resource::<T>(parameters) {
            return Some(Handle::new(id));
        }

        if let Some(resource) = T::load(parameters, self) {
            let id = self.next_id;
            self.next_id += 1;

            let storage = self.storage::<T>();
            storage.insert_resource(id, resource, parameters.clone());

            return Some(Handle::new(id));
        }

        None
    }

    fn find_cached_resource<T: Resource>(&self, parameters: &T::Parameters) -> Option<ResourceId> {
        let storage = self.storages.get(&TypeId::of::<T>())?;
        let storage = storage.downcast_ref::<ResourceStorage<T>>()?;
        if let Some(&id) = storage.parameters_cache.get(parameters) {
            return Some(id);
        }

        None
    }
}

impl Default for ResourceManager {
    fn default() -> Self {
        Self::new()
    }
}
