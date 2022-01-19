use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[actix_rt::main]
async fn main() -> Result<(), actix_web::Error> {
    HttpServer::new(move || App::new().service(index))
        .bind("127.0.0.1:8081")?
        .run()
        .await?;
    Ok(())
}

#[get("/")]
async fn index() -> Result<impl Responder, actix_web::Error> {
    let response_body = "Hello, World!!";

    Ok(HttpResponse::Ok().body(response_body))
}