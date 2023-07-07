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
async fn new_show_test() {
    use crate::handlers::shows::new_show;
    use crate::models::shows::CreateShowBody;

    let show_body = CreateShowBody {
        owner_id: "99999".to_string(),
        title: "Sample Show".to_string(),
        description: "Sample Description".to_string(),
        view_code: None,
    };

    let app = init::init("/shows", new_show).await;
    let req = test::TestRequest::post()
        .uri("/shows")
        .set_json(&show_body)
        .to_request();

    let res = test::call_service(&app, req).await;

    // let body = test::read_body(res).await;
    // let body_str = String::from_utf8_lossy(&body);
    // println!("Response body: {}", body_str);
    // assert!(false);
    assert!(res.status().is_success(), "Failed to create a new show");
}

#[actix_web::test]
async fn get_show_by_id_test() {
    use crate::handlers::shows::get_show_by_id;

    let app = init::init("/shows", get_show_by_id).await;
    // NEED TO FIND A WAY TO GET SHOW ID FOR TESTING
    let req = test::TestRequest::get().uri("/shows").to_request();

    let res = test::call_service(&app, req).await;

    // let body = test::read_body(res).await;
    // let body_str = String::from_utf8_lossy(&body);
    // println!("Response body: {}", body_str);
    // assert!(false);
    assert!(res.status().is_success(), "Failed to get shows");
}

#[actix_web::test]
async fn edit_show_test() {
    use crate::handlers::shows::edit_show;
    use crate::models::shows::UpdateShowBody;

    let show_body = UpdateShowBody {
        title: Some("Change Show".to_string()),
        description: Some("Change Description".to_string()),
        view_code: None,
    };

    let app = init::init("/shows", edit_show).await;
    let req = test::TestRequest::post()
        // NEED TO FIND A WAY TO GET SHOW ID FOR TESTING
        .uri("/shows")
        .set_json(&show_body)
        .to_request();

    let res = test::call_service(&app, req).await;

    // let body = test::read_body(res).await;
    // let body_str = String::from_utf8_lossy(&body);
    // println!("Response body: {}", body_str);
    // assert!(false);
    assert!(res.status().is_success(), "Failed to create a new show");
}

#[actix_web::test]
async fn delete_show_test() {
    use crate::handlers::shows::delete_show;
    use crate::models::shows::DeleteShowParams;

    let show_body = DeleteShowParams {
        owner_id: "99999".to_string(),
    };

    let app = init::init("/shows", delete_show).await;
    let req = test::TestRequest::post()
        // NEED TO FIND A WAY TO GET SHOW ID FOR TESTING
        .uri("/shows")
        .set_json(&show_body)
        .to_request();

    let res = test::call_service(&app, req).await;

    // let body = test::read_body(res).await;
    // let body_str = String::from_utf8_lossy(&body);
    // println!("Response body: {}", body_str);
    // assert!(false);
    assert!(res.status().is_success(), "Failed to create a new show");
}

#[actix_web::test]
async fn get_all_user_shows_test() {
    use crate::handlers::shows::get_all_user_shows;
    use crate::models::shows::GetUserShowsParams;

    let show_body = GetUserShowsParams {
        favorites: Some(false),
    };

    let app = init::init("/shows", get_all_user_shows).await;
    let req = test::TestRequest::post()
        // NEED TO FIND A WAY TO GET SHOW ID FOR TESTING
        .uri("/shows")
        .set_json(&show_body)
        .to_request();

    let res = test::call_service(&app, req).await;

    // let body = test::read_body(res).await;
    // let body_str = String::from_utf8_lossy(&body);
    // println!("Response body: {}", body_str);
    // assert!(false);
    assert!(res.status().is_success(), "Failed to create a new show");
}
