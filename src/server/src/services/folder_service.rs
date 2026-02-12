use axum::Json;
use diesel::RunQueryDsl;

use crate::{config::db::DbPool, models::folder::Folder, schema::folders};

pub struct FolderService {
    pool: DbPool,
}

impl FolderService {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub fn list_folders(&self) -> Result<Json<Vec<Folder>>, String> {
        let mut connection = self.pool.get().expect("Failed to get connection from pool");

        match folders::table.load::<Folder>(&mut connection) {
            Ok(value) => Ok(Json(value)),
            Err(error) => Err(error.to_string()),
        }
    }
}
