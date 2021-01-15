use actix_web::test::TestRequest;
use assert2::check;
use insta::assert_json_snapshot;
#[actix_rt::test]
async fn check_health_is_successful() {
    let service = crate::Service::new().await;

    let response = service.inject(TestRequest::get().uri("/health").to_request()).await;

    check!(response.status == 200);
    assert_json_snapshot!(response.to_json().unwrap(), @r###"
    {
      "healthy": true,
      "components": {
        "db": {
          "healthy": true
        }
      }
    }
    "###);
}
