pub mod db;

pub mod core {
    pub mod sync;
}

use crate::core::sync::SyncCommand;
use clap::{command, Command};

fn main() {
    let matches = command!()
        .subcommand(
            Command::new("sync")
                .about("Synchronyzes packages database with the upstream one."),
        )
        .get_matches();

    if let Some(_matches) = matches.subcommand_matches("sync") {
        return SyncCommand::new().configure();
    }
}
