use actix_web::test::TestRequest;
use assert2::check;
use insta::assert_json_snapshot;
use patina_testdatabase::seed::SeedUser;

#[actix_rt::test]
async fn get_user_given_unknown_user_is_not_found() {
    let service = crate::Service::new().await;

    let response = service
        .inject(
            TestRequest::get()
                .uri("/users/3f92b3b0-4716-449a-a159-beabf3b59d99")
                .to_request(),
        )
        .await;

    check!(response.status == 404);
    check!(response.headers.get("content-type").unwrap() == "application/problem+json");
    check!(response.headers.get("cache-control").unwrap() == "no-store");
    assert_json_snapshot!(response.to_json().unwrap(), @r###"
    {
      "type": "https://httpstatuses.com/404",
      "title": "Not Found",
      "status": 404
    }
    "###);
}

#[actix_rt::test]
async fn get_user_given_invalid_id_is_not_found() {
    let service = crate::Service::new().await;

    let response = service
        .inject(TestRequest::get().uri("/users/not_a_uuid").to_request())
        .await;

    check!(response.status == 404);
    check!(response.headers.get("content-type").unwrap() == "application/problem+json");
    check!(response.headers.get("cache-control").unwrap() == "no-store");
    assert_json_snapshot!(response.to_json().unwrap(), @r###"
    {
      "type": "https://httpstatuses.com/404",
      "title": "Not Found",
      "status": 404
    }
    "###);
}

#[actix_rt::test]
async fn get_user_given_known_id_is_returned() {
    let seed_user = SeedUser {
        user_id: "384a7b7f-8ec2-4f73-9dae-4eb4f7b178b3".parse().unwrap(),
        version: "a76b376a-9ca9-4b90-bb20-c5c5133d2ba7".parse().unwrap(),
        display_name: "Test User".to_string(),
        email: "test@example.com".to_string(),
        ..SeedUser::default()
    }
    .with_authentication("fake", "123456", "test@example.com");

    let service = crate::Service::new().await;
    service.seed(&seed_user).await;

    let response = service
        .inject(
            TestRequest::get()
                .uri("/users/384a7b7f-8ec2-4f73-9dae-4eb4f7b178b3")
                .to_request(),
        )
        .await;

    check!(response.status == 200);
    check!(response.headers.get("content-type").unwrap() == "application/hal+json");
    check!(response.headers.get("cache-control").unwrap() == "public, max-age=3600");
    check!(response.headers.get("etag").unwrap() == "\"a76b376a-9ca9-4b90-bb20-c5c5133d2ba7\"");
    assert_json_snapshot!(response.to_json().unwrap(), @r###"
    {
      "displayName": "Test User",
      "email": "test@example.com",
      "authentications": [
        {
          "service": "fake",
          "userId": "123456",
          "displayName": "test@example.com"
        }
      ],
      "_links": {
        "self": {
          "href": "/users/384a7b7f-8ec2-4f73-9dae-4eb4f7b178b3"
        }
      }
    }
    "###);
}
