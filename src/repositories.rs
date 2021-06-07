use lazy_static::lazy_static;
use std::{collections::HashMap, sync::{Arc, Mutex}};

lazy_static! {
    static ref URLS: Arc<Mutex<HashMap<String, String>>> = {
        Arc::new(Mutex::new(HashMap::new()))
    };
}

pub fn get_url_by_id(id: &str) -> Result<String, &str> {
    match URLS.lock().unwrap().get(id) {
        None => Err("id not found"),
        Some(x) => Ok(x.clone())
    }
}

pub fn insert_url(id: &str, url: &str) {
    URLS
        .lock()
        .unwrap()
        .insert(String::from(id), String::from(url));
}