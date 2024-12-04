use std::{
    fs::{self, File},
    io::Read,
    path::Path,
};

use bincode::{deserialize, serialize};
use serde::{Deserialize, Serialize};

use crate::graph::Collection;

pub trait Store<T> {
    fn write<U>(&self, data: Collection<U>) -> Result<(), String>
    where
        U: Serialize + for<'a> Deserialize<'a>;
    fn read<U>(&self) -> Result<Collection<U>, String>
    where
        U: Serialize + for<'a> Deserialize<'a>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinaryStore {
    path: String,
}

impl BinaryStore {
    pub fn new(path: &str) -> Self {
        BinaryStore {
            path: path.to_string(),
        }
    }

    fn ensure_file_exists(&self) -> Result<(), String> {
        let path = Path::new(&self.path);
        if !path.exists() {
            File::create(&self.path).map_err(|e| format!("Failed to create file: {}", e))?;
        }
        Ok(())
    }
}

impl<T> Store<T> for BinaryStore
where
    T: Serialize + for<'a> Deserialize<'a>,
{
    fn write<U>(&self, data: Collection<U>) -> Result<(), String>
    where
        U: Serialize + for<'a> Deserialize<'a>,
    {
        self.ensure_file_exists()?;

        let binary_data =
            serialize(&data).map_err(|e| format!("Failed to serialize data: {}", e))?;

        fs::write(&self.path, binary_data).map_err(|e| format!("Failed to write file: {}", e))?;
        Ok(())
    }

    fn read<U>(&self) -> Result<Collection<U>, String>
    where
        U: Serialize + for<'a> Deserialize<'a>,
    {
        self.ensure_file_exists()?;

        let mut file = File::open(&self.path).map_err(|e| format!("Failed to open file: {}", e))?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)
            .map_err(|e| format!("Failed to read file: {}", e))?;

        if buffer.is_empty() {
            return Err("File is empty".to_string());
        }

        let collection: Collection<U> =
            deserialize(&buffer).map_err(|e| format!("Failed to deserialize data: {}", e))?;

        Ok(collection)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonStore {
    path: String,
}

impl JsonStore {
    pub fn new(path: &str) -> Self {
        JsonStore {
            path: path.to_string(),
        }
    }

    fn ensure_file_exists(&self) -> Result<(), String> {
        let path = Path::new(&self.path);
        if !path.exists() {
            File::create(&self.path).map_err(|e| format!("Failed to create file: {}", e))?;
        }
        Ok(())
    }
}

impl<T> Store<T> for JsonStore
where
    T: Serialize + for<'a> Deserialize<'a>,
{
    fn write<U>(&self, data: Collection<U>) -> Result<(), String>
    where
        U: Serialize + for<'a> Deserialize<'a>,
    {
        self.ensure_file_exists()?;
        let json = serde_json::to_string(&data).map_err(|e| e.to_string())?;

        fs::write(self.path.clone(), json).map_err(|e| e.to_string())?;

        Ok(())
    }

    fn read<U>(&self) -> Result<Collection<U>, String>
    where
        U: Serialize + for<'a> Deserialize<'a>,
    {
        let json = std::fs::read_to_string(self.path.clone()).map_err(|e| e.to_string())?;

        let collection = serde_json::from_str(&json).map_err(|e| e.to_string())?;

        Ok(collection)
    }
}
