use actix_web::{App, HttpServer};
use todo_api::controllers;
use anyhow::Result;

#[actix_web::main]
async fn main() -> Result<()> {
    HttpServer::new(|| App::new().service(controllers::get_todos))
        .bind("127.0.0.1:8081")?
        .run()
        .await?;
    Ok(())
}
