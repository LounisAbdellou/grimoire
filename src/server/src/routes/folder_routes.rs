use axum::{Router, routing::get};

use crate::{config::db::DbPool, services::folder_service::FolderService};

pub fn router(pool: DbPool) -> Router {
    let service = FolderService::new(pool);

    Router::new().route("/", get(service.list_folders()))
}
