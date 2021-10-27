use legion::*;
use std::collections::HashMap;
use crate::ResState::InMemory;
use std::sync::Arc;

pub enum ResState {
    Undefined,
    OnDisk,
    InMemory,
    InGPUMemory
}

pub trait Resource {
    fn GetState(&self) -> ResState;
    fn new_arc(path : &String) -> Arc<Self> where Self: Sized;
}

pub struct ResMesh {

}

pub struct ResTexture {

}

pub struct ResPBRCluster {

}

pub struct ResPipeline {

}

impl ResMesh {
    pub fn new(str : &String) -> ResMesh {
        Self {

        }
    }
}

impl Resource for ResMesh {
    fn GetState(&self) -> ResState {
        todo!()
    }

    fn new_arc(path: &String) -> Arc<ResMesh> {
        Arc::new(ResMesh::new(path))
    }
}

impl ResTexture {
    pub fn new(str : &String) -> Self {
        Self {

        }
    }
}

impl Resource for ResTexture {
    fn GetState(&self) -> ResState {
        todo!()
    }

    fn new_arc(path: &String) -> Arc<Self> where Self: Sized {
        Arc::new(ResTexture::new(path))
    }
}

pub struct ResourceManager {
    mesh_map : HashMap<String, Arc<ResMesh>>,
    texture_map : HashMap<String, Arc<ResTexture>>,
    pbr_cluster_map : HashMap<String, Arc<ResPBRCluster>>,
    pipeline_map : HashMap<String, Arc<ResPipeline>>,
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
            mesh_map : HashMap::new(),
            texture_map : HashMap::new(),
            pbr_cluster_map : HashMap::new(),
            pipeline_map : HashMap::new(),
        }
    }

    fn get_mesh(&mut self, uri : &String) -> Arc<ResMesh> {
        ResourceManager::get::<ResMesh>(&mut self.mesh_map, uri)
    }

    fn get_texture(&mut self, uri : &String) -> Arc<ResTexture> {
        ResourceManager::get::<ResTexture>(&mut self.texture_map, uri)
    }

    fn get<T>(map : &mut HashMap<String, Arc<T>>, str : &String) -> Arc<T> where T: Resource {
        let res = map.get(str);
        match res {
             Some(val) => Arc::clone(val),
            None => {
                let rs = T::new_arc(str);
                let cl = Arc::clone(&rs);
                map.insert(str.to_string(), rs);
                cl
            }
        }
    }
}