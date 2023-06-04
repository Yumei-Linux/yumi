pub mod db;
pub mod repository;

pub mod util {
    pub mod exec;
    pub mod metadata;
    pub mod confirm;
    pub mod fetch;
}

pub mod core {
    pub mod query;
    pub mod sync;
    pub mod grab;
}

use std::process::exit;

use crate::core::sync::Sync;
use crate::core::grab::Grab;
use clap::{command, Command, Arg, ArgAction};

#[tokio::main]
async fn main() {
    let matches = command!()
        .subcommand(
            Command::new("sync")
                .about("Synchronyzes packages database with the upstream one."),
        )
        .subcommand(
            Command::new("grab")
                .about("Grab and merges a new package into the system by fetching it from the Yumi's pkgs databases")
                .arg(Arg::new("pkgs").action(ArgAction::Append))
        )
        .get_matches();

    if let Some(_matches) = matches.subcommand_matches("sync") {
        return Sync::new().configure();
    }

    if let Some(matches) = matches.subcommand_matches("grab") {
        let pkgs = matches
            .get_many::<String>("pkgs")
            .unwrap_or_default()
            .map(|v| v.to_string())
            .collect::<Vec<_>>();

        Grab::new(pkgs).install_pkgs().await.unwrap_or_else(|error| {
            println!("Cannot grab pkgs: {}", error.to_string());
            exit(1);
        });

        return;
    }
}
