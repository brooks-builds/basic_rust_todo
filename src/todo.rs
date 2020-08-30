#[derive(Debug)]
pub struct Todo {
    item: String,
    id: u32,
    pub completed: bool,
}

impl Todo {
    /// The line coming in will look like
    /// `my todo item|false`
    pub fn new(line: String, id: u32) -> Todo {
        let mut item_iterator = line.split('|');
        let item = item_iterator.next().unwrap_or("").to_owned();
        let completed = match item_iterator.next().unwrap_or("false") {
            "true" => true,
            "false" => false,
            _ => false,
        };

        Todo {
            item,
            id,
            completed,
        }
    }

    pub fn print(&self) -> String {
        let completed = if self.completed { 'x' } else { ' ' };
        format!("{} [{}] - {}", self.id, completed, self.item)
    }

    pub fn print_for_file(&self) -> String {
        format!("{}|{}", self.item, self.completed)
    }
}
