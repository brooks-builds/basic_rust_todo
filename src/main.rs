use basic_rust_todo;
use std::env::args;

fn main() {
    let arguments = args().skip(1).collect();
    let todo_items = basic_rust_todo::run("todos.data", arguments).unwrap();

    println!("Todo Items");
    println!("----------");
    for item in todo_items {
        println!("{}", item.print());
    }
}
