use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn run(file_name: &'static str) -> Result<Vec<String>, String> {
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
}

fn format_todo_item(todo_item: String) -> String {
    format!("[ ] - {}", todo_item)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_run_the_app() {
        let file_name = "todos.data";
        let todo_items: Vec<String> = run(file_name).unwrap();
        assert_eq!(2, todo_items.len());
        assert_eq!("[ ] - make a todo list application", todo_items[0]);
        assert_eq!("[ ] - play the drop game", todo_items[1]);
    }
}