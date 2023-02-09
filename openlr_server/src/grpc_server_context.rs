use openlr::{map::Map, decoding_parameters::DecodingParameters};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use url::Url;

pub struct GrpcServerContext {
    pub mdbs: Arc<Mutex<HashMap<Url,Arc<dyn Map>>>>,
    pub params: Mutex<HashMap<String, Arc<DecodingParameters>>>,
}

impl GrpcServerContext {
    pub fn new() -> Self {
        GrpcServerContext {
            mdbs: Arc::new(Mutex::new(HashMap::new())),
            params: Mutex::new(HashMap::from([(
                "default".to_owned(),
                Arc::new(DecodingParameters::default()),
            )])),
        }
    }

    pub async fn add_map_database(&mut self, key:Url, value: Arc<dyn Map>) {
        self.mdbs.lock().await.insert(key,value);
    }

    pub async fn add_param_set(&mut self, key:String, value: DecodingParameters) {
        self.params.lock().await.insert(key, Arc::new(value));
    }

    pub async fn get_param_set(&self, key:&String) -> Option<Arc<DecodingParameters>>  {
        match self.params.lock().await.get(key) {
            Some(params) => Some(params.clone()),
            _ =>   None
        }
    }
}

unsafe impl Sync for GrpcServerContext {}
