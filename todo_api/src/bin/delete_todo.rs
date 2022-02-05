use diesel::prelude::*;
use todo_api::schema::todo as todo_schema;
use todo_api::utils::establish_connection;
use anyhow::Result;

fn main() -> Result<()> {
    let connection = establish_connection().expect("error getting conn");
    diesel::delete(todo_schema::dsl::todo.find(1))
        .execute(&connection)
        .expect("Error deleting todo");
    Ok(())
}