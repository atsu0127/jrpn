use crate::schema::todo;
use serde::{ Serialize, Deserialize };

#[derive(Insertable)]
#[table_name = "todo"]
pub struct NewTodo {
    pub title: String,
    pub content: String,
}

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Todo {
    pub id: i32,
    pub title: Option<String>,
    pub content: Option<String>,
}