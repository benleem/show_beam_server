use super::init;
#[cfg(test)]
use actix_web::test;

#[actix_web::test]
async fn get_all_shows_test() {
    use crate::handlers::shows::get_all_shows;

    let app = init::init("/shows", get_all_shows).await;
    let req = test::TestRequest::get().uri("/shows").to_request();

    let res = test::call_service(&app, req).await;

    // let body = test::read_body(res).await;
    // let body_str = String::from_utf8_lossy(&body);
    // println!("Response body: {}", body_str);
    // assert!(false);
    assert!(res.status().is_success(), "Failed to get shows");
}

#[actix_web::test]
async fn shows_crud_integration_test() {
    let id = new_show_test().await;
    assert!(get_all_user_shows_test().await);
    assert!(get_show_by_id_test(&id).await);
    assert!(edit_show_test(&id).await);
    assert!(delete_show_test(&id).await);
}

#[allow(unused)]
async fn new_show_test() -> String {
    use crate::handlers::shows::new_show;
    use crate::models::shows::CreateShowBody;
    use actix_web::test;

    let show_body = CreateShowBody {
        title: "Sample Show".to_string(),
        description: "Sample Description".to_string(),
        public: true,
        view_code: None,
    };

    let app = init::init("/shows", new_show).await;
    let req = test::TestRequest::post()
        .uri("/shows")
        .set_json(&show_body)
        .to_request();

    let res = test::call_service(&app, req).await;

    let body = test::read_body(res).await;
    let json_body: serde_json::Value = serde_json::from_slice(&body).unwrap();

    json_body["data"]["show"]["id"]
        .as_str()
        .unwrap()
        .to_string()
}

#[allow(unused)]
async fn get_all_user_shows_test() -> bool {
    use crate::handlers::shows::get_all_user_shows;
    use crate::models::shows::GetUserShowsParams;
    use actix_web::test;

    let show_body = GetUserShowsParams { favorites: false };

    let user_id = "99999".to_string();
    let app = init::init("/shows", get_all_user_shows).await;
    let req = test::TestRequest::get()
        .uri(&format!("/shows/users/{user_id}"))
        .set_json(&show_body)
        .to_request();

    let res = test::call_service(&app, req).await;

    let body = test::read_body(res).await;
    let body_str = String::from_utf8_lossy(&body);
    println!("Get User Shows Response body: {}", body_str);
    body_str.contains("success")
}

#[allow(unused)]
async fn get_show_by_id_test(id: &str) -> bool {
    use crate::handlers::shows::get_show_by_id;
    use actix_web::test;
    let app = init::init("/shows", get_show_by_id).await;

    let req = test::TestRequest::get()
        .uri(&format!("/shows/{id}"))
        .to_request();

    let res = test::call_service(&app, req).await;

    let body = test::read_body(res).await;
    let body_str = String::from_utf8_lossy(&body);
    println!("Get Response body: {}", body_str);
    body_str.contains("success")
}

#[allow(unused)]
async fn edit_show_test(id: &str) -> bool {
    use crate::handlers::shows::edit_show;
    use crate::models::shows::UpdateShowBody;
    use actix_web::test;

    let show_body = UpdateShowBody {
        title: Some("Change Show".to_string()),
        description: Some("Change Description".to_string()),
        public: Some(false),
        view_code: None,
    };

    let app = init::init("/shows", edit_show).await;
    let req = test::TestRequest::patch()
        .uri(&format!("/shows/{id}"))
        .set_json(&show_body)
        .to_request();

    let res = test::call_service(&app, req).await;

    let body = test::read_body(res).await;
    let body_str = String::from_utf8_lossy(&body);
    println!("Edit Response body: {}", body_str);
    body_str.contains("success")
}

#[allow(unused)]
async fn delete_show_test(id: &str) -> bool {
    use crate::handlers::shows::delete_show;
    // use crate::models::shows::DeleteShowParams;
    use actix_web::test;

    // let show_body = DeleteShowParams {
    //     owner_id: "99999".to_string(),
    // };

    let app = init::init("/shows", delete_show).await;
    let req = test::TestRequest::delete()
        .uri(&format!("/shows/{id}"))
        // .set_json(&show_body)
        .to_request();

    let res = test::call_service(&app, req).await;

    let body = test::read_body(res).await;
    let body_str = String::from_utf8_lossy(&body);
    println!("Delete Response body: {}", body_str);
    body_str == ""
}
