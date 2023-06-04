use std::path::{PathBuf, Path};
use crate::util::{metadata::{Metadata, Data}, confirm::confirm, exec, fetch::fetch_url};
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

    async fn download_sources(&self, metadata: &Data) {
        for (name, url) in metadata.downloads.clone().into_iter() {
            println!("Downloading source: {}...", name);

            let url = url.to_string().replace("\"", "");

            if let Err(err) = fetch_url(url, name.to_string()).await {
                println!("Cannot download {}: {}", name, err.to_string());
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
            println!("FATAL: Cannot find the functions file! this has gone terribly wrong... please recheck your configure");
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

    async fn install_pkg(&self, pkg: &str) -> Result<(), String> {
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

        Query::show_pkg_info(&parsed_metadata);

        if confirm(format!("Do you wish to merge this package? ({})", parsed_metadata.metadata.name).as_str()) {
            self.download_sources(&parsed_metadata).await;
            self.run_builder(pkg, &pkg_paths);
        }

        Ok(())
    }

    pub async fn install_pkgs(&self) -> Result<(), String> {
        for pkg in &self.pkgs {
            if let Err(err) = self.install_pkg(pkg).await {
                return Err(err);
            }
        }

        Ok(())
    }
}
