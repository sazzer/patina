use actix_http::cookie::Cookie;
use actix_web::test::TestRequest;
use assert2::{check, let_assert};
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

    let cookies: Vec<Cookie> = response
        .headers
        .get_all("set-cookie")
        .map(|value| value.to_str().unwrap())
        .map(|value| Cookie::parse(value).unwrap())
        .collect();

    let authentication_provider = cookies
        .iter()
        .find(|c| c.name() == "authentication_provider");
    let_assert!(Some(authentication_provider) = authentication_provider);
    check!(authentication_provider.value() == "google");
    check!(authentication_provider.http_only() == Some(true));

    let authentication_nonce = cookies.iter().find(|c| c.name() == "authentication_nonce");
    let_assert!(Some(authentication_nonce) = authentication_nonce);
    check!(authentication_nonce.http_only() == Some(true));

    check!(
        response.headers.get("location").unwrap()
            == &format!("https://accounts.google.com/o/oauth2/v2/auth?client_id=GoogleClientId&response_type=code&scope=openid%20email%20profile&redirect_uri=http%3A%2F%2Fexample.com%2Fauthentication%2Fgoogle%2Fredirect&state={}", authentication_nonce.value())
    );
}
