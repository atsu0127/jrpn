use crate::repositories;
use actix_web::{ HttpResponse, Responder, get };

#[get("/todos")]
pub async fn get_todos() -> impl Responder {
    println!("called get_todos");
    match repositories::get_todos() {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(err) => {
            eprintln!("error: {:?}", err);
            HttpResponse::InternalServerError().finish()
        },
    }
}