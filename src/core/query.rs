use crate::util::metadata::Data;

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
}