use diesel::prelude::*;
use todo_api::schema::todo as todo_schema;
use todo_api::utils::establish_connection;

fn main() {
    let connection = establish_connection();
    diesel::update(todo_schema::dsl::todo.find(2))
        .set(todo_schema::title.eq("update_title"))
        .execute(&connection)
        .expect("Error updating todo");
}
