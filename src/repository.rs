use std::path::Path;

use crate::util::exec::exec;

pub struct Repository {
    id: String,
}

impl Repository {
    pub fn new(id: String) -> Self {
        Self {
            id
        }
    }

    pub fn clone_at(&self, path: &Path) -> Result<(), String> {
        let cmd = format!(
            "git clone {} {}",
            self.id, path.to_string_lossy().to_string()
        );

        exec(cmd)
    }
}