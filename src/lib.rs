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
            let new_todo = Todo::new(format!("{}|false", new_todo_item), 0);
            write_all_to_file(file_name, vec![new_todo], true)?;
            list_all(file_name)
        }
        Command::Toggle(id) => {
            let mut todos = list_all(file_name)?;
            if todos.len() <= id {
                return Err("Error, provided id not associated with a todo item".to_owned());
            }
            todos[id].toggle_completion();
            write_all_to_file(file_name, todos, false)?;

            list_all(file_name)
        }
        Command::Remove(id) => {
            let mut todos = list_all(file_name)?;
            if todos.len() <= id {
                return Err("Error, provided id not associated with a todo item".to_owned());
            }
            todos.remove(id);
            write_all_to_file(file_name, todos, false)?;
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

fn write_all_to_file(file_name: &str, todos: Vec<Todo>, append: bool) -> Result<(), String> {
    let mut file = OpenOptions::new();
    file.write(true);

    if append {
        file.append(true);
    } else {
        file.truncate(true);
    }

    let mut file = match file.open(file_name) {
        Ok(file) => file,
        Err(error) => return Err(format!("error opening file for writing: {}", error)),
    };
    if let Err(error) = todos
        .iter()
        .try_for_each(|todo| write!(file, "{}\n", todo.print_for_file()))
    {
        return Err(format!("error writing todos to file: {}", error));
    }

    Ok(())
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
            "make a todo list application|false\nplay the drop game|true\na new todo item|false\n",
            std::fs::read_to_string(file_name).unwrap()
        );
    }

    fn reset_todos_data() {
        let mut todos_file = File::create("todos.data").unwrap();
        let todo_items = "make a todo list application|false\nplay the drop game|true\n".to_owned();
        todos_file.write(todo_items.as_bytes()).unwrap();
    }

    #[test]
    fn can_mark_todo_item_as_completed() {
        reset_todos_data();
        let arguments = vec!["toggle".to_owned(), "0".to_owned()];
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
        let arguments = vec!["toggle".to_owned(), "1".to_owned()];
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

    #[test]
    fn test_not_providing_id() {
        reset_todos_data();
        let arguments = vec!["toggle".to_owned()];
        let file_name = "todos.data";
        assert_eq!("error, missing id", run(file_name, arguments).unwrap_err());
    }

    #[test]
    fn test_not_a_number_as_an_id() {
        reset_todos_data();
        let arguments = vec!["toggle".to_owned(), "a".to_owned()];
        let file_name = "todos.data";
        assert_eq!(
            "Error parsing id out of the command: invalid digit found in string",
            run(file_name, arguments).unwrap_err()
        );
    }

    #[test]
    fn test_id_too_high_for_toggle() {
        reset_todos_data();
        let arguments = vec!["toggle".to_owned(), "5".to_owned()];
        let file_name = "todos.data";
        assert_eq!(
            "Error, provided id not associated with a todo item",
            run(file_name, arguments).unwrap_err()
        );
    }

    #[test]
    fn test_handling_lack_of_add_command() {
        reset_todos_data();
        let arguments = vec!["add".to_owned()];
        let file_name = "todos.data";

        assert_eq!(
            "error, missing new todo item",
            run(file_name, arguments).unwrap_err()
        );
    }
}
