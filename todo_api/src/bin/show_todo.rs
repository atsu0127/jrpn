use diesel::prelude::*;
use todo_api::models::Todo;
use todo_api::schema::todo as todo_schema;
use todo_api::utils::establish_connection;

fn main() {
    let connection = establish_connection().expect("error getting conn");
    let todo = todo_schema::dsl::todo
        .load::<Todo>(&connection)
        .expect("Error loading todo");

    for t in todo {
        println!("{:?}", t);
    }
}