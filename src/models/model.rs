use crate::schema::users;
use diesel::prelude::*;
use diesel::Insertable;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Selectable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub email: String,
}
