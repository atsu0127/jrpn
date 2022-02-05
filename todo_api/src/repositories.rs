use crate::utils;
use crate::models::{NewTodo, Todo};
use crate::schema::todo as todo_schema;
use diesel::prelude::*;
use anyhow::{Context, Result};

pub fn get_todos(id: Option<i32>) -> Result<Vec<Todo>> {
    let connection = utils::establish_connection()?;
    let mut query = todo_schema::dsl::todo.into_boxed();
    if let Some(id) = id {
        query = query.filter(todo_schema::id.eq(id));
    }

    let todo = query
        .load::<Todo>(&connection)
        .with_context(|| {
           "load error".to_string()
        })?;

    Ok(todo)
}

pub fn insert_todo(todo: NewTodo) -> Result<()> {
    let connection = utils::establish_connection()?;
    diesel::insert_into(todo_schema::dsl::todo)
        .values(&todo)
        .execute(&connection)
        .with_context(|| {
            format!("insert error. todo: {}", todo.title)
        })?;
    Ok(())
}