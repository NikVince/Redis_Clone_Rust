use bytes::Bytes;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// Define the database type
pub type Db = Arc<Mutex<HashMap<String, Bytes>>>;

// Create a new empty database
pub fn new() -> Db {
    Arc::new(Mutex::new(HashMap::new()))
}
