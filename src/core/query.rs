use std::{fs, path::{PathBuf, Path}};

use crate::util::metadata::{Data, Metadata};

pub struct Query;

impl Query {
    pub fn show_pkg_info(metadata: &Data) {
        println!("Showing package metainfo for package {}...", metadata.metadata.name);

        println!("  * Name: {}", metadata.metadata.name);
        println!("  * Description: {}", metadata.metadata.description);
        println!("  * Version: {}", metadata.metadata.version);

        if metadata.metadata.provides.len() > 0 {
            println!("  * Provides:");
            for file in &metadata.metadata.provides {
                println!("    -> {}", file);
            }
        }

        if metadata.metadata.deps.len() > 0 {
            println!("  * Depends on:");
            for dep in &metadata.metadata.deps {
                println!("    -> {}", dep);
            }
        }

        if metadata.downloads.len() > 0 {
            println!("  * Downloads:");
            for (name, url) in metadata.downloads.clone().into_iter() {
                println!("    -> {}: {}", name, url.to_string().replace("\"", ""));
            }
        }
    }

    pub fn pkg_is_installed(id: String) -> bool {
        let filepath = format!("/var/yumi/{}", id);
        let root_path = Path::new(filepath.as_str());

        if !root_path.is_dir() {
            println!("[E] Fatal: Root path {} isn't a directory", root_path.to_string_lossy().to_string());
            std::process::exit(1);
        }

        let mut metadata_path = PathBuf::from(root_path);
        metadata_path.push("metadata.toml");
        if !metadata_path.is_file() {
            println!("[E] Fatal: {} No such file or directory", metadata_path.to_string_lossy().to_string());
        }

        let metadata = Metadata::new(metadata_path.to_string_lossy().to_string());

        let parsed_metadata = match metadata.parse() {
            Ok(value) => value,
            Err(err) => {
                println!("Cannot parse metadata of the pkg {}: {}", id, err.to_string());
                std::process::exit(1);
            },
        };

        return Query::is_installed(&parsed_metadata);
    }

    pub fn is_installed(metadata: &Data) -> bool {
        if metadata.metadata.provides.len() == 0 {
            return false;
        }

        for file in &metadata.metadata.provides {
            if let Ok(_) = fs::metadata(file) {
                return true;
            }
        }

        return false;
    }
}