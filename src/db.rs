use std::{path::{PathBuf, Path}};

use crate::repository::Repository;

pub struct DB {
    pub path: PathBuf
}

impl DB {
    pub fn new() -> Self {
        Self {
            path: DB::obtain_path()
        }
    }

    fn obtain_path() -> PathBuf {
        PathBuf::from(Path::new("/var/yumi"))
    }

    pub fn fetch_packages(&self) -> Result<(), String> {
        let path = self.path.as_path();

        println!("[I] Starting database updating");

        if path.is_dir() {
            println!("  * Removing older database indexes");
            std::fs::remove_dir_all(path.to_string_lossy().to_string()).unwrap_or_else(|error| {
                println!("[Fatal] Cannot update database due to {}", error.to_string());
                std::process::exit(1);
            });
        }

        println!("[I] Updating yumi-packages into {}", path.to_string_lossy().to_string());
    
        let url = "https://github.com/Yumei-Linux/yumi-packages.git";

        Repository::new(url.to_string())
            .clone_at(path)
    }
}