use diesel::prelude::*;
use todo_api::utils;
use todo_api::models::NewTodo;
use todo_api::schema::todo as todo_schema;

fn main() {
    let connection = utils::establish_connection().expect("error getting conn");
    let new_todos = vec![
        NewTodo {
            title: "new title2".to_string(),
            content: "new content2".to_string(),
        },
        NewTodo {
            title: "new title3".to_string(),
            content: "new content3".to_string(),
        }
    ];
    diesel::insert_into(todo_schema::dsl::todo)
        .values(&new_todos)
        .execute(&connection)
        .expect("Error saving new todo");
}