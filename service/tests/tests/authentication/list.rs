use actix_web::test::TestRequest;
use assert2::check;
use insta::assert_json_snapshot;

#[actix_rt::test]
async fn list_providers_is_successful() {
    let service = crate::Service::new().await;

    let response = service
        .inject(TestRequest::get().uri("/authentication").to_request())
        .await;

    check!(response.status == 200);
    assert_json_snapshot!(response.to_json().unwrap(), @r###"
    {
      "_links": {
        "self": {
          "href": "/authentication"
        },
        "tag:patina,2021,rels/authentication/start": [
          {
            "href": "/authentication/google",
            "name": "google"
          },
          {
            "href": "/authentication/twitter",
            "name": "twitter"
          }
        ]
      }
    }
    "###);
}
