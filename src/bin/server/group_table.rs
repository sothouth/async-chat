use crate::group::Group;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct GroupTable(Mutex<HashMap<Arc<String>, Arc<Group>>>);

impl GroupTable {
    pub fn new() -> Self {
        GroupTable(Mutex::new(HashMap::new()))
    }
    pub fn get(&self, key: &String) -> Option<Arc<Group>> {
        self.0.lock().unwrap().get(key).cloned()
    }
    pub fn get_or_create(&self, key: Arc<String>) -> Arc<Group> {
        self.0
            .lock()
            .unwrap()
            .entry(key.clone())
            .or_insert_with(|| Arc::new(Group::new()))
            .clone()
    }
}
