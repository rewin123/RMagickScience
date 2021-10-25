use legion::*;
use async_std::sync::Arc;
use std::collections::HashMap;
use crate::ResState::InMemory;

pub enum ResState {
    Undefined,
    OnDisk,
    InMemory,
    InGPUMemory
}

pub trait Resource<T> {
    fn GetState(&self) -> ResState;
    fn new(path : String) -> Arc<Resource<T>> where Self: Sized;
}

pub struct StringResource {
    path : String,
    state : ResState
}

impl Resource<StringResource> for StringResource {
    fn GetState(&self) -> ResState {
        todo!()
    }

    fn new(path: String) -> Arc<Resource<StringResource>> where Self: Sized {
        Arc::new(
            Self {
                path,
                state : InMemory
            })
    }
}

pub struct ResourceManager {
    resource_table : HashMap<String, Arc<dyn Resource<()>>>
}

pub struct Logic {
    world : World
}

impl Logic {
    pub fn new() -> Logic {
        Self {
            world : World::default()
        }
    }

    pub fn Step(&self, dt : f32) {

    }
}

impl ResourceManager {
    pub fn new() -> ResourceManager {
        Self {
            resource_table : HashMap::new()
        }
    }
}