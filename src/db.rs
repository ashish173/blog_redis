use bytes::Bytes;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub struct Db {
    pub entries: Arc<Mutex<HashMap<String, Bytes>>>,
}

impl Db {
    pub fn new() -> Db {
        Db {
            entries: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn write(&self, arr: &[String]) -> Result<&str, &'static str> {
        let key = &arr[1];
        let value = &arr[2];

        // we need to clone the referenced value since Bytes::from() function expects a 'static lifetime
        // variable but `value` has unknown lifetime in this function context
        let val = value.clone();
        let p = &self
            .entries
            .lock()
            .unwrap()
            .insert(String::from(key), Bytes::from(val));

        match p {
            Some(p) => Ok("r Ok"), // if they key was already present
            None => Ok("Ok"),      // if the key was not present
        }
    }

    /// Reads data from the database
    pub fn read(&self, arr: &[String]) -> Result<Bytes, &'static str> {
        let key = &arr[1];
        let query_result = &self.entries.lock().unwrap();
        let res = query_result.get(key);
        match res {
            Some(value) => Ok(value.clone()),
            None => Err("no such key found"),
        }
    }
}
