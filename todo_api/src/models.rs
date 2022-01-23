use crate::schema::todo;

#[derive(Insertable)]
#[table_name = "todo"]
pub struct NewTodo {
    pub title: String,
    pub content: String,
}

#[derive(Debug, Queryable)]
pub struct Todo {
    pub id: i32,
    pub title: Option<String>,
    pub content: Option<String>,
}