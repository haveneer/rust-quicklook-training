use routes_dsl::routes;

fn get_users() -> String {
    "List of users".to_string()
}

fn create_user() -> String {
    "User created".to_string()
}

fn get_user() -> String {
    "User details".to_string()
}

routes! {
    GET "/users" => get_users,
    POST "/users" => create_user,
    GET "/users/:id" => get_user
}

#[test]
fn test_routes() {
    let routes = setup_routes();
    assert_eq!(routes.len(), 3);

    assert_eq!(routes[0].0, "GET");
    assert_eq!(routes[0].1, "/users");
    assert_eq!((routes[0].2)(), "List of users");

    assert_eq!(routes[1].0, "POST");
    assert_eq!(routes[1].1, "/users");
}
