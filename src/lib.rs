mod command;
mod todo;

use command::Command;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::BufReader;
use todo::Todo;

pub fn run(file_name: &'static str, arguments: Vec<String>) -> Result<Vec<Todo>, String> {
    let command = Command::new(arguments)?;
    match command {
        Command::ListAll => list_all(file_name),
        Command::Add(new_todo_item) => {
            let mut todo_file = match OpenOptions::new().append(true).open(file_name) {
                Ok(file) => file,
                Err(error) => return Err(format!("error opening file for writing: {}", error)),
            };
            match write!(todo_file, "{}|false\r\n", new_todo_item) {
                Err(error) => return Err(format!("error writing to file: {}", error)),
                Ok(_) => list_all(file_name),
            }
        }
        Command::Done(id) => {
            let mut todos = list_all(file_name)?;
            todos[id].completed = true;
            let mut todo_file = match OpenOptions::new().write(true).open(file_name) {
                Ok(file) => file,
                Err(error) => {
                    return Err(format!(
                        "error opening file for writing a completed todo: {}",
                        error
                    ))
                }
            };
            if let Err(error) = todos
                .iter()
                .try_for_each(|todo| write!(todo_file, "{}\r\n", todo.print_for_file()))
            {
                return Err(format!("error writing completed todo to disk: {}", error));
            }

            list_all(file_name)
        }
        Command::Remove(id) => {
            let mut todos = list_all(file_name)?;
            todos.remove(id);
            let mut todo_file = match OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(file_name)
            {
                Ok(file) => file,
                Err(error) => {
                    return Err(format!(
                        "error opening file for writing a completed todo: {}",
                        error
                    ))
                }
            };
            dbg!(&todos);
            if let Err(error) = todos
                .iter()
                .try_for_each(|todo| write!(todo_file, "{}\r\n", todo.print_for_file()))
            {
                return Err(format!("error writing completed todo to disk: {}", error));
            }
            list_all(file_name)
        }
        Command::Uncheck(id) => {
            let mut todos = list_all(file_name)?;
            todos[id].completed = false;
            let mut todo_file = match OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(file_name)
            {
                Ok(file) => file,
                Err(error) => {
                    return Err(format!(
                        "error opening file for writing a completed todo: {}",
                        error
                    ))
                }
            };
            if let Err(error) = todos
                .iter()
                .try_for_each(|todo| write!(todo_file, "{}\r\n", todo.print_for_file()))
            {
                return Err(format!("error writing completed todo to disk: {}", error));
            }

            list_all(file_name)
        }
    }
}

fn list_all(file_name: &str) -> Result<Vec<Todo>, String> {
    let todo_file = match File::open(file_name) {
        Ok(file) => file,
        Err(error) => return Err(format!("error opening file: {}", error)),
    };
    let todo_file_reader = BufReader::new(todo_file);
    let mut todo_items = vec![];
    for line in todo_file_reader.lines() {
        match line {
            Ok(todo_item) => todo_items.push(Todo::new(todo_item, todo_items.len() as u32)),
            Err(error) => return Err(format!("error reading lines from todo file: {}", error)),
        }
    }
    Ok(todo_items)
}

fn format_todo_item(todo_item: String) -> String {
    format!("[ ] - {}", todo_item)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_list_all_todo_items() {
        reset_todos_data();
        let file_name = "todos.data";
        let arguments = vec![];
        let todo_items: Vec<Todo> = run(file_name, arguments).unwrap();
        assert_eq!(2, todo_items.len());
        assert_eq!(
            "0 [ ] - make a todo list application",
            todo_items[0].print()
        );
        assert_eq!("1 [x] - play the drop game", todo_items[1].print());
    }

    #[test]
    fn can_add_new_todo_item() {
        reset_todos_data();
        let file_name = "todos.data";
        let todo_items_before_add = run(file_name, vec![]).unwrap();
        assert_eq!(2, todo_items_before_add.len());
        let arguments = vec!["add".to_owned(), "a new todo item".to_owned()];
        run(file_name, arguments).unwrap();
        let todo_items_after_add = run(file_name, vec![]).unwrap();

        assert_eq!(3, todo_items_after_add.len());
        assert_eq!(
            "make a todo list application|false\r\nplay the drop game|true\r\na new todo item|false\r\n",
            std::fs::read_to_string(file_name).unwrap()
        );
    }

    fn reset_todos_data() {
        let mut todos_file = File::create("todos.data").unwrap();
        let todo_items =
            "make a todo list application|false\r\nplay the drop game|true\r\n".to_owned();
        todos_file.write(todo_items.as_bytes()).unwrap();
    }

    #[test]
    fn can_mark_todo_item_as_completed() {
        reset_todos_data();
        let arguments = vec!["done".to_owned(), "0".to_owned()];
        let file_name = "todos.data";
        match run(file_name, arguments) {
            Ok(todo_items) => assert_eq!(
                "0 [x] - make a todo list application",
                todo_items[0].print()
            ),
            Err(error) => eprintln!(
                "error testing if we can mark todo items as completed: {}",
                error
            ),
        }
    }

    #[test]
    fn can_mark_todo_item_as_not_completed() {
        reset_todos_data();
        let arguments = vec!["uncheck".to_owned(), "1".to_owned()];
        let file_name = "todos.data";
        match run(file_name, arguments) {
            Ok(todo_items) => assert_eq!("1 [ ] - play the drop game", todo_items[1].print()),
            Err(error) => eprintln!(
                "error testing if we can mark todo items as completed: {}",
                error
            ),
        }
    }

    #[test]
    fn test_delete_todo_item() {
        reset_todos_data();
        let arguments = vec!["delete".to_owned(), "0".to_owned()];
        let file_name = "todos.data";
        match run(file_name, arguments) {
            Ok(todo_items) => {
                assert_eq!(1, todo_items.len());
                assert_eq!("0 [x] - play the drop game", todo_items[0].print());
            }
            Err(error) => eprintln!("error testing deleting items: {}", error),
        }
    }
}
