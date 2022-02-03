use crate::utils;
use crate::models::Todo;
use crate::schema::todo as todo_schema;
use diesel::prelude::*;
use anyhow::{Context, Result};

pub fn get_todos() -> Result<Vec<Todo>> {
    let connection = utils::establish_connection()?;
    let todo = todo_schema::dsl::todo
        .load::<Todo>(&connection)
        .with_context(|| {
           "load error".to_string()
        })?;

    Ok(todo)
}

pub fn get_todo_by_id(id: u32) -> Result<Vec<Todo>> {
    let connection = utils::establish_connection()?;
    let todo = todo_schema::dsl::todo
        .find(id)
        .load::<Todo>(&connection)
        .with_context(|| {
            "load error".to_string()
        })?;

    Ok(todo)
}