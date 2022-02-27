use crate::repositories;
use actix_web::{HttpResponse, Responder, get, post, patch, delete, web};
use crate::models::NewTodo;
use crate::repositories::SearchInfo;

#[get("/todos")]
pub async fn get_todos(web::Query(info): web::Query<SearchInfo>) -> impl Responder {
    println!("called get_todos");
    match repositories::get_todos(None, Some(info)) {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(err) => {
            eprintln!("error: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/todos/{id}")]
pub async fn get_todo_by_id(web::Path(id): web::Path<i32>) -> impl Responder {
    println!("called get_todo_by_id: {}", id);
    match repositories::get_todos(Some(id), None) {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(err) => {
            eprintln!("error: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
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

#[patch("/todos/{id}")]
pub async fn put_todo(web::Path(id): web::Path<i32>, web::Json(todo): web::Json<NewTodo>) -> impl Responder {
    println!("patch todo: {:?}", todo);
    match repositories::update_todo(id, todo) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => {
            eprintln!("error: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[delete("/todos/{id}")]
pub async fn delete_todo(web::Path(id): web::Path<i32>) -> impl Responder {
    println!("put todo: {:?}", id);
    match repositories::delete_todo(id) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => {
            eprintln!("error: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}