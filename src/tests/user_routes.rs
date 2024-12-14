#[cfg(test)]
mod tests {
    use crate::routes::get_routes;
    use crate::utils::db::init_db_pool;
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
}
