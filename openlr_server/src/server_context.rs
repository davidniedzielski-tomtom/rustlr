use openlr::{map::Map, decoding_parameters::DecodingParameters};
use std::{collections::HashMap, sync::{Mutex, Arc}};
use url::Url;

pub struct ServerContext {
    pub mdbs: Mutex<HashMap<Url, Arc<dyn Map>>>,
    pub params: Mutex<HashMap<String, Arc<DecodingParameters>>>,
}

unsafe impl Sync for ServerContext {}

impl ServerContext {
    pub fn new() -> Self {
        ServerContext {
            mdbs: Mutex::new(HashMap::new()),
            params: Mutex::new(HashMap::from([(
                "default".to_owned(),
                Arc::new(DecodingParameters::default()),
            )])),
        }
    }

    pub fn add_map_database(&mut self, key:Url, value: Arc<dyn Map>) {
        self.mdbs.lock().unwrap().insert(key,value);
    }

    pub fn add_param_set(&mut self, key:String, value: DecodingParameters) {
        self.params.lock().unwrap().insert(key, Arc::new(value));
    }

    pub fn get_param_set(&self, key:&String) -> Option<Arc<DecodingParameters>>  {
        match self.params.lock().unwrap().get(key) {
            Some(params) => Some(params.clone()),
            _ =>   None
        }
    }
}
