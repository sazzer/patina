use actix_web::test::TestRequest;
use assert2::check;
use insta::assert_json_snapshot;

#[actix_rt::test]
async fn get_user_given_unknown_user_is_not_found() {
    let service = crate::Service::new().await;

    let response = service
        .inject(TestRequest::get().uri("/users/3f92b3b0-4716-449a-a159-beabf3b59d99").to_request())
        .await;

    check!(response.status == 404);
    check!(response.headers.get("content-type").unwrap() == "application/problem+json");
    check!(response.headers.get("cache-control").unwrap() == "no-store");
    assert_json_snapshot!(response.to_json().unwrap(), @r###"
    {
      "type": "tag:patina/2020:problems/not_found",
      "title": "The requested resource was not found",
      "status": 404
    }
    "###);
}

#[actix_rt::test]
async fn get_user_given_invalid_id_is_not_found() {
    let service = crate::Service::new().await;

    let response = service.inject(TestRequest::get().uri("/users/not_a_uuid").to_request()).await;

    check!(response.status == 404);
    check!(response.headers.get("content-type").unwrap() == "application/problem+json");
    check!(response.headers.get("cache-control").unwrap() == "no-store");
    assert_json_snapshot!(response.to_json().unwrap(), @r###"
    {
      "type": "tag:patina/2020:problems/not_found",
      "title": "The requested resource was not found",
      "status": 404
    }
    "###);
}
