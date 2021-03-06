use crate::utils;
use crate::models::{NewTodo, Todo};
use crate::schema::todo as todo_schema;
use diesel::prelude::*;
use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SearchInfo {
    title: Option<String>,
    content: Option<String>,
}

pub fn get_todos(id: Option<i32>, info: Option<SearchInfo>) -> Result<Vec<Todo>> {
    let connection = utils::establish_connection()?;
    let mut query = todo_schema::dsl::todo.into_boxed();
    if let Some(id) = id {
        query = query.filter(todo_schema::id.eq(id));
    }

    if let Some(info) = info {
        if let Some(title) = info.title {
            query = query.filter(todo_schema::title.like(format!("%{}%", title)));
        }

        if let Some(content) = info.content {
            query = query.filter(todo_schema::content.like(format!("%{}%", content)));
        }
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

pub fn update_todo(id: i32, todo: NewTodo) -> Result<()> {
    let connection = utils::establish_connection()?;
    diesel::update(todo_schema::dsl::todo.filter(todo_schema::id.eq(id)))
        .set(&todo)
        .execute(&connection)
        .with_context(|| {
            format!("update error. todo: {}", id)
        })?;
    Ok(())
}

pub fn delete_todo(id: i32) -> Result<()> {
    let connection = utils::establish_connection()?;
    diesel::delete(todo_schema::dsl::todo.filter(todo_schema::id.eq(id)))
        .execute(&connection)
        .with_context(|| {
            format!("delete error. todo: {}", id)
        })?;
    Ok(())
}