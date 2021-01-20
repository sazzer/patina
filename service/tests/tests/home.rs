use actix_web::test::TestRequest;
use assert2::check;
use insta::assert_json_snapshot;

#[actix_rt::test]
async fn home_document_is_successful() {
    let service = crate::Service::new().await;

    let response = service
        .inject(TestRequest::get().uri("/").to_request())
        .await;

    check!(response.status == 200);
    check!(response.headers.get("content-type").unwrap() == "application/hal+json");
    check!(response.headers.get("cache-control").unwrap() == "public, max-age=3600");

    assert_json_snapshot!(response.to_json().unwrap(), @r###"
    {
      "_links": {
        "tag:patina,2021:rels/authentication": {
          "href": "/authentication"
        },
        "tag:patina,2021:rels/health": {
          "href": "/health"
        }
      }
    }
    "###);
}
