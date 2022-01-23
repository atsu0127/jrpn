use diesel::prelude::*;
use todo_api::schema::todo as todo_schema;
use todo_api::utils::establish_connection;

fn main() {
    let connection = establish_connection();
    diesel::delete(todo_schema::dsl::todo.find(1))
        .execute(&connection)
        .expect("Error deleting todo");
}