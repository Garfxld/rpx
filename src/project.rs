#![allow(dead_code)]

use std::{fs, path::{Path, PathBuf}};

use anyhow::Result;


pub struct Project {
    path: PathBuf
}


impl Project {
    
    pub fn open(path: &Path) -> Result<Self> {
        Ok(Self {
            path: path.to_path_buf()
        })
    }

    pub fn version(&self) -> i32 {
        3
    }

    pub fn path(&self) -> PathBuf {
        self.path.clone()
    } 

    pub fn assets_path(&self) -> PathBuf {
        self.path.join("assets")
    }

    pub fn get_namespace(&self, namespace: &str) -> PathBuf {
        self.assets_path().join(namespace)
    }

    pub fn has_namespace(&self, namespace: &str) -> bool {
        self.get_namespace(namespace).exists()
    }

    pub fn create_namespace(&self, namespace: &str) -> Result<()> {
        let path = self.get_namespace(namespace);
        fs::create_dir(path)?;

        Ok(())
    }

}