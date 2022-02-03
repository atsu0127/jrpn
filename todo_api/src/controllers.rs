use crate::repositories;
use actix_web::{HttpResponse, Responder, get, web};

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

#[get("/todos/{id}")]
pub async fn get_todo_by_id(web::Path(id): web::Path<i32>) -> impl Responder {
    println!("called get_todo_by_id: {}", id);
    match repositories::get_todo_by_id(id) {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(err) => {
            eprintln!("error: {:?}", err);
            HttpResponse::InternalServerError().finish()
        },
    }
}