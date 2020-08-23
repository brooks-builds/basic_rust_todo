use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::BufReader;
use eyre::{Result};
use std::fmt::Display;

enum Command {
    ListAll,
    Add(String)
}

impl Command {
    pub fn new(arguments: Vec<String>) -> Result<Command, String> {
        if arguments.is_empty() {
            return Ok(Command::ListAll);
        }

        if arguments.len() == 1 {
            return Err("Missing argument".to_owned());
        }

        match arguments[0].to_lowercase().as_ref() {
            "add" => Ok(Command::Add(arguments[1].clone())),
            _ => Ok(Command::ListAll)
        }
    }
}

pub fn run(file_name: &'static str, arguments: Vec<String>) -> Result<Vec<String>, String> {
    let command = Command::new(arguments)?;
    
    match command {
        Command::ListAll => {
            let todo_file = File::open(file_name).unwrap();
            let todo_file_reader = BufReader::new(todo_file);
            let mut todo_items = vec![];
            for line in todo_file_reader.lines() {
                match line {
                    Ok(todo_item) => todo_items.push(format_todo_item(todo_item)),
                    Err(error) => return Err(format!("error reading lines from todo file: {}", error))
                }
            }
            Ok(todo_items)
        },
        Command::Add(new_todo_item) => {
            let mut todo_file = match OpenOptions::new().append(true).open(file_name) {
                Ok(file) => file,
                Err(error) => return Err(format!("error opening file for writing: {}", error))
            };
            match todo_file.write(format!("\r\n{}", new_todo_item).as_bytes()) {
                Err(error) => return Err(format!("error writing to file: {}", error)),
                Ok(_) => Ok(vec![format!("Added {}", new_todo_item)])
            }
            
        }
    }
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
        let todo_items: Vec<String> = run(file_name, arguments).unwrap();
        assert_eq!(2, todo_items.len());
        assert_eq!("[ ] - make a todo list application", todo_items[0]);
        assert_eq!("[ ] - play the drop game", todo_items[1]);
    }

    #[test]
    fn can_add_new_todo_item() {
        reset_todos_data();
        let file_name = "todos.data";
        let todo_items_before_add = run(file_name, vec![]).unwrap();
        assert_eq!(2, todo_items_before_add.len());
        let arguments = vec![
            "add".to_owned(),
            "a new todo item".to_owned()
        ];
        run(file_name, arguments).unwrap();
        let todo_items_after_add = run(file_name, vec![]).unwrap();
        assert_eq!(3, todo_items_after_add.len());
    }

    fn reset_todos_data() {
        let mut todos_file = File::create("todos.data").unwrap();
        let todo_items = "make a todo list application\r\nplay the drop game".to_owned();
        todos_file.write(todo_items.as_bytes()).unwrap();
    }
}