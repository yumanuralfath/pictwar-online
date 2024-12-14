use crate::models::users::{NewUser, User};
use crate::schema::users::dsl::*;
use crate::utils::db::DbPool;
use diesel::prelude::*;
use rocket::serde::json::Json;
use rocket::State;

#[get("/users")]
pub async fn get_users(pool: &State<DbPool>) -> Json<Vec<User>> {
    let mut conn = pool.get().expect("Failed to get DB connection");
    let results = users.load::<User>(&mut conn).expect("Error loading users");
    Json(results)
}

#[get("/user/<user_id>")]
pub async fn get_user(user_id: i32, pool: &State<DbPool>) -> Json<User> {
    let mut conn = pool.get().expect("Failed to get DB connection");
    let user = users
        .find(user_id)
        .first(&mut conn)
        .expect("Error loading user");
    Json(user)
}

#[post("/users", data = "<user>")]
pub async fn create_user(user: Json<NewUser>, pool: &State<DbPool>) -> Json<User> {
    let mut conn = pool.get().expect("Failed to get DB connection");
    let new_user = diesel::insert_into(users)
        .values(user.into_inner())
        .get_result(&mut conn)
        .expect("Error creating user");
    Json(new_user)
}
