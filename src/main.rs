use basic_rust_todo;

fn main() {
    let todo_items = basic_rust_todo::run("todos.data").unwrap();

    println!("Todo Items");
    println!("----------");
    for item in todo_items {
        println!("{}", item);
    }
}
