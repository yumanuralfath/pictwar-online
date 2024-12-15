#[cfg(test)]
mod tests {
    use crate::routes::get_routes;
    use crate::utils::db::init_db_pool;
    use rocket::http::ContentType;
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    fn init_client() -> Client {
        let rocket = rocket::build()
            .manage(init_db_pool())
            .mount("/", get_routes());
        Client::tracked(rocket).expect("valid rocket instance")
    }

    #[test]
    fn test_get_users() {
        let client = init_client();
        let response = client.get("/users").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_create_user() {
        let client = init_client();
        let new_user =
            r#"{"username": "John Doe", "email": "john@example.com", "password": "password"}"#;
        let response = client
            .post("/users")
            .header(ContentType::JSON)
            .body(new_user)
            .dispatch();
        assert_eq!(response.status(), Status::Created);
    }
}
