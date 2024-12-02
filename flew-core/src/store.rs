use std::{
    fs::{self, File},
    io::Read,
    path::Path,
};

use bincode::deserialize;
use serde::{Deserialize, Serialize};

pub trait Store<T> {
    fn save(&self, data: T);
    fn load(&self) -> T;
}

pub struct BinaryStore {
    path: String,
}

impl BinaryStore {
    pub fn new(path: &str) -> Self {
        BinaryStore {
            path: path.to_string(),
        }
    }
}

impl<T> Store<T> for BinaryStore
where
    T: Serialize + for<'a> Deserialize<'a>,
{
    fn save(&self, data: T) {
        let existing_data = if Path::new(&self.path).exists() {
            let json = fs::read_to_string(&self.path).expect("failed to read");
            serde_json::from_str::<Vec<T>>(&json).unwrap_or_else(|_| Vec::new())
        // Deserialize or create empty Vec if file is empty
        } else {
            Vec::new() // If the file doesn't exist, create a new Vec
        };

        // Append the new data
        let mut updated_data = existing_data;
        updated_data.push(data);

        // Serialize the updated data and write it back to the file
        let json = serde_json::to_string(&updated_data).expect("failed to serialize");
        let _ = fs::write(&self.path, json);
    }
    fn load(&self) -> T {
        let mut file = File::open(self.path.clone()).expect("Failed to open file");
        let mut buff = Vec::new();
        file.read_to_end(&mut buff).expect("failed to read");
        if buff.is_empty() {
            panic!("File is empty")
        }
        deserialize(&buff).expect("failed to deserialize")
    }
}

pub struct JsonStore {
    path: String,
}

impl JsonStore {
    pub fn new(path: &str) -> Self {
        JsonStore {
            path: path.to_string(),
        }
    }
}

impl<T> Store<T> for JsonStore
where
    T: Serialize + for<'a> Deserialize<'a>,
{
    fn save(&self, data: T) {
        let json = serde_json::to_string(&data).expect("failed to serialize");
        std::fs::write(self.path.clone(), json).expect("failed to write");
    }
    fn load(&self) -> T {
        let json = std::fs::read_to_string(self.path.clone()).expect("failed to read");
        serde_json::from_str(&json).expect("failed to deserialize")
    }
}
