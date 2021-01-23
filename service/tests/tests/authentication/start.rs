use actix_web::test::TestRequest;
use assert2::check;
use insta::assert_json_snapshot;

#[actix_rt::test]
async fn start_authentication_when_unknown_provider_is_error() {
    let service = crate::Service::new().await;

    let response = service
        .inject(
            TestRequest::get()
                .uri("/authentication/unknown")
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
async fn start_authentication_when_using_google_is_redirect() {
    let service = crate::Service::new().await;

    let response = service
        .inject(
            TestRequest::get()
                .uri("/authentication/google")
                .to_request(),
        )
        .await;

    check!(response.status == 303);
    check!(response.headers.get("content-type") == None);
    check!(response.headers.get("location") != None);
}
