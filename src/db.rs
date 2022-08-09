use bytes::Bytes;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub struct Db {
    // pub entries: Arc<bool>,
    pub entries: Arc<Mutex<HashMap<String, Bytes>>>,
}
impl Db {
    pub fn new() -> Db {
        Db {
            // entries: Arc::new(true),
            entries: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn write(&self, arr: &[String]) -> Result<&str, &'static str> {
        let key = &arr[1];
        let value = &arr[2];

        // we need to clone the referenced value since Bytes::from() function expects a 'static lifetime
        // variable but `value` has unknown lifetime in this function context
        let val = value.clone();
        println!("lockAquired");
        // let p = &self.entries.insert(String::from(key), Bytes::from(val));
        let p = &self
            .entries
            .lock()
            .unwrap()
            // .get(key);
            .insert(String::from(key), Bytes::from(val));
        // .insert(String::from(key), Bytes::from(val));
        println!("data written=={}", key);
        match p {
            Some(p) => {
                println!("P==={:?}", p);
                Ok("r Ok")
            }, // if they key was already present
            None => Ok("Ok"), // if the key was not present
        }
        // Ok("")
    }

    /// Reads data from the database
    pub fn read(&self, arr: &[String]) -> Result<Bytes, &'static str> {
        let key = &arr[1];
        println!("lockAquired");
        let query_result = &self.entries.lock().unwrap();
        // .get(key);
        // let result  = query_result.as_ref().unwrap();
        let res = query_result.get(key);
        // .unwrap().get(key);
        // let query_clone =  query_result.get(key);
        match res {
            Some(value) => {
                // let cloned_val = value.clone();
                Ok(value.clone())
                // Ok(Bytes::from("Hello"))
            },
            None => Err("no such key found")
        }
        // Ok(Bytes::from("Hello"))

        // if let Some(value) = query_result {
        //     let cloned_value = value.clone();
        //     return Ok(cloned_value);
        // } else {
        //     return Err("no such key found");
        // }
    }
}
