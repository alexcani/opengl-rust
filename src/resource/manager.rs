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

    pub fn load_resource<T: Resource>(&mut self, parameters: &T::Parameters) -> Option<Handle<T>> {
        if let Some(id) = self.find_cached_resource::<T>(parameters) {
            return Some(Handle::new(id));
        }

        if let Some(resource) = T::load(parameters, self) {
            let id = self.next_id;
            self.next_id += 1;

            let storage = self.storage_mut::<T>();
            storage.insert_resource(id, resource, parameters.clone());

            return Some(Handle::new(id));
        }

        None
    }

    pub fn get<T: Resource>(&self, id: ResourceId) -> Option<&T> {
        let storage = self.storage::<T>()?;
        storage.resources.get(&id)
    }

    pub fn get_mut<T: Resource>(&mut self, id: ResourceId) -> Option<&mut T> {
        let storage = self.storage_mut::<T>();
        storage.resources.get_mut(&id)
    }

    /// Get a mutable reference to the resource storage for a specific type
    /// Creates a new storage if it doesn't exist
    fn storage_mut<T: Resource>(&mut self) -> &mut ResourceStorage<T> {
        let type_id = TypeId::of::<T>();
        self.storages
            .entry(type_id)
            .or_insert(Box::new(ResourceStorage::<T>::new()))
            .downcast_mut()
            .unwrap()
    }

    /// Get an immutable reference to the resource storage for a specific type
    /// Returns None if the storage doesn't exist
    fn storage<T: Resource>(&self) -> Option<&ResourceStorage<T>> {
        let type_id = TypeId::of::<T>();
        self.storages.get(&type_id)?.downcast_ref::<ResourceStorage<T>>()
    }

    fn find_cached_resource<T: Resource>(&self, parameters: &T::Parameters) -> Option<ResourceId> {
        let storage = self.storage::<T>()?;
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
