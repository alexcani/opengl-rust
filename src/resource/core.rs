use std::{hash::Hash, marker::PhantomData};

use super::manager::ResourceManager;

pub type ResourceId = u64;

pub trait Resource: Sized + 'static {
    type Parameters: Clone + Hash + Eq;

    fn load(parameters: &Self::Parameters, manager: &ResourceManager) -> Option<Self>;
    fn unload(&mut self);
}

pub struct Handle<T: Resource> {
    id: ResourceId,
    _marker: PhantomData<T>,
}

impl<T: Resource> Handle<T> {
    pub fn new(id: ResourceId) -> Self {
        Self {
            id,
            _marker: PhantomData,
        }
    }

    pub fn id(&self) -> ResourceId {
        self.id
    }
}
