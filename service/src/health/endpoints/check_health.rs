use actix_web::Responder;

pub async fn check_health() -> impl Responder {
    "Hello, World!"
}
