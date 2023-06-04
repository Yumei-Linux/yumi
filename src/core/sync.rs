use crate::db::DB;

pub struct Sync {
    db: DB
}

impl Sync {
    pub fn new() -> Self {
        Self {
            db: DB::new()
        }
    }

    pub fn configure(&self) {
        if let Err(err) = self.db.fetch_packages() {
            println!("[Fatal] Cannot synchronyze database: {}", err.to_string());
            std::process::exit(1);
        }
    }
}