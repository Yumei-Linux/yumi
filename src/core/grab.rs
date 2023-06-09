use std::path::{PathBuf, Path};
use crate::util::{metadata::{Metadata, Data}, confirm::confirm, exec};
use super::query::Query;

pub struct Grab {
    pkgs: Vec<String>,
}

struct PkgPaths {
    root: PathBuf,
    builder: PathBuf,
    metadata: PathBuf
}

impl PkgPaths {
    pub fn from_root(root: PathBuf) -> Self {
        let path = root.as_path();
        Self {
            root: PathBuf::from(path),
            builder: PkgPaths::construct_path(path, "builder.sh"),
            metadata: PkgPaths::construct_path(path, "metadata.toml"),
        }
    }

    fn construct_path(root_path: &Path, appendix: &str) -> PathBuf {
        let mut path = PathBuf::from(root_path);
        path.push(appendix);

        return path;
    }
}

impl Grab {
    pub fn new(pkgs: Vec<String>) -> Self {
        Self {
            pkgs,
        }
    }

    fn pkg_paths(&self, pkg: &str) -> PkgPaths {
        let root = PathBuf::from(Path::new(
            format!("/var/yumi/{}", pkg).as_str()
        ));
        
        PkgPaths::from_root(root)
    }

    fn download_sources(&self, metadata: &Data) {
        for (name, url) in metadata.downloads.clone().into_iter() {
            println!("Downloading source: {}...", name);

            let url = url.to_string().replace("\"", "");

            if let Err(err) = exec::exec(format!("wget {}", url)) {
                println!("Cannot download source {}: {}", url, err.to_string());
                std::process::exit(1);
            }

            let command = format!(
                "mkdir -pv ./.yumi-downloads && mv -v {} ./.yumi-downloads",
                name
            );

            exec::exec(command).unwrap_or_else(|error| {
                println!("Cannot download the source!: {}", error.to_string());
                std::process::exit(1);
            });
        }
    }

    fn run_builder(&self, pkg: &str, pkg_paths: &PkgPaths) {
        let functions_path = Path::new("/var/yumi/functions.sh");
        if !functions_path.is_file() {
            println!("FATAL: Cannot find the functions file! this has gone terribly wrong... please recheck your configuration");
            std::process::exit(1);
        }

        let functions = functions_path
            .to_string_lossy()
            .to_string();

        let builder = pkg_paths.builder
            .to_string_lossy()
            .to_string();

        if let Err(err) = exec::exec(format!("bash {} {}", functions, builder)) {
            println!("Cannot build pkg {}: {}", pkg, err.to_string());
            std::process::exit(1);
        }

        println!("* Building finished for package {}", pkg);
    }

    fn resolve_dependencies(&self, pkg: String, parsed_metadata: &Data) -> Result<(), String> {
        if parsed_metadata.metadata.deps.len() > 0 {
            for dep in &parsed_metadata.metadata.deps {
                if let Err(err) = self.install_pkg(dep, false) {
                    return Err(format!("Cannot compile dependency: {} for package {}: {}", dep, pkg, err.to_string()));
                }
            }
        }

        Ok(())
    }

    fn install_pkg(&self, pkg: &str, interactive: bool) -> Result<(), String> {
        let pkg_paths = self.pkg_paths(pkg);
        if !pkg_paths.root.is_dir() {
            return Err(format!("Yumi wasn't able to find {}, sorry for the inconvenients.", pkg));
        }

        let metadata = Metadata::new(pkg_paths.metadata.to_string_lossy().to_string());

        let parsed_metadata = match metadata.parse() {
            Ok(value) => value,
            Err(err) => {
                println!("{}", err.to_string());
                std::process::exit(1);
            },
        };

        if interactive {
            Query::show_pkg_info(&parsed_metadata);
            if Query::pkg_is_installed(pkg.to_string()) {
                println!("[warn] this package is already installed, rebuilding...");
            }
        } else if !interactive && Query::pkg_is_installed(pkg.to_string()) {
            println!("[warn] {} is already installed, skipping... (pass -f to rebuild the dependencies even if they are already installed)", pkg);
            return Ok(());
        }

        if !interactive {
            self.resolve_dependencies(pkg.to_string(), &parsed_metadata).unwrap_or_else(|error| {
                println!("{}", error.to_string());
                std::process::exit(1);
            });

            self.download_sources(&parsed_metadata);
            self.run_builder(pkg, &pkg_paths);

            return Ok(());
        }

        // this is gonna run just on interactive mode.
        if confirm(format!("Do you wish to merge this package? ({})", parsed_metadata.metadata.name).as_str()) {
            self.resolve_dependencies(pkg.to_string(), &parsed_metadata).unwrap_or_else(|error| {
                println!("{}", error.to_string());
                std::process::exit(1);
            });

            self.download_sources(&parsed_metadata);
            self.run_builder(pkg, &pkg_paths);
        }

        Ok(())
    }

    pub fn install_pkgs(&self) -> Result<(), String> {
        for pkg in &self.pkgs {
            if let Err(err) = self.install_pkg(pkg, true) {
                return Err(err);
            }
        }

        Ok(())
    }
}
