use crate::models::users::{NewUser, User};
use crate::services::user_service::UserService; // Tambahkan ini
use crate::utils::db::DbPool;
use rocket::serde::json::Json;
use rocket::State;

#[get("/users")]
pub async fn get_users(pool: &State<DbPool>) -> Json<Vec<User>> {
    let user_service = UserService::new(pool.inner());
    let results = user_service.get_users();
    Json(results)
}

#[get("/user/<user_id>")]
pub async fn get_user(user_id: i32, pool: &State<DbPool>) -> Json<User> {
    let user_service = UserService::new(pool.inner());
    let user = user_service.get_user(user_id);
    Json(user)
}

#[post("/users", data = "<user>")]
pub async fn create_user(user: Json<NewUser>, pool: &State<DbPool>) -> Json<User> {
    let user_service = UserService::new(pool.inner());
    let new_user = user_service.create_user(user.into_inner());
    Json(new_user)
}
