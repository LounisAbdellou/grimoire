use crate::schema::folders;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Queryable, Selectable, Identifiable, Clone)]
#[diesel(table_name = folders)]
pub struct Folder {
    pub id: i64,
    pub name: String,
}
