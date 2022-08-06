use bytes::Bytes;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Db {
    pub entries: HashMap<String, Bytes>,
}
impl Db {
    pub fn new() -> Db {
        Db {
            entries: HashMap::new(),
        }
    }

    pub fn write(&mut self, arr: &[String]) -> Result<&str, &'static str> {
        let key = &arr[1];
        let value = &arr[2];

        // we need to clone the referenced value since Bytes::from() function expects a 'static lifetime
        // variable but `value` has unknown lifetime in this function context
        let val = value.clone();

        let p = &self.entries.insert(String::from(key), Bytes::from(val));

        match p {
            Some(_p) => Ok("r Ok"), // if they key was already present
            None => Ok("Ok"),       // if the key was not present
        }
    }

    /// Reads data from the database
    pub fn read(&mut self, arr: &[String]) -> Result<&Bytes, &'static str> {
        let key = &arr[1];
        let query_result = self.entries.get(key);

        if let Some(value) = query_result {
            return Ok(value);
        } else {
            return Err("no such key found");
        }
    }
}
