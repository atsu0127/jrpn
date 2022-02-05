use crate::repositories;
use actix_web::{HttpResponse, Responder, get, post, web};
use crate::models::NewTodo;

#[get("/todos")]
pub async fn get_todos() -> impl Responder {
    println!("called get_todos");
    match repositories::get_todos(None) {
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
    match repositories::get_todos(Some(id)) {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(err) => {
            eprintln!("error: {:?}", err);
            HttpResponse::InternalServerError().finish()
        },
    }
}

#[post("/todos")]
pub async fn post_todo(todo: web::Json<NewTodo>) -> impl Responder {
    let todo = todo.into_inner();
    println!("post todo: {:?}", todo);
    match repositories::insert_todo(todo) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => {
            eprintln!("error: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}