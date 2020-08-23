use basic_rust_todo;
use std::env::ArgsOs;

fn main() {
    let arguments = vec![];
    let todo_items = basic_rust_todo::run("todos.data", arguments).unwrap();

    println!("Todo Items");
    println!("----------");
    for item in todo_items {
        println!("{}", item);
    }
}
