pub enum Command {
    ListAll,
    Add(String),
    Toggle(usize),
    Remove(usize),
}

impl Command {
    pub fn new(arguments: Vec<String>) -> Result<Command, String> {
        if arguments.is_empty() {
            return Ok(Command::ListAll);
        }

        match arguments[0].to_lowercase().as_ref() {
            "add" => {
                if arguments.len() == 1 {
                    Err("error, missing new todo item".to_owned())
                } else {
                    Ok(Command::Add(arguments[1].clone()))
                }
            }
            "toggle" => Ok(Command::Toggle(Self::get_id_from_arguments(arguments)?)),
            "delete" => Ok(Command::Remove(Self::get_id_from_arguments(arguments)?)),
            _ => Ok(Command::ListAll),
        }
    }

    fn get_id_from_arguments(arguments: Vec<String>) -> Result<usize, String> {
        if arguments.len() == 1 {
            return Err("error, missing id".to_owned());
        }

        match arguments[1].parse::<usize>() {
            Ok(id) => Ok(id),
            Err(error) => Err(format!("Error parsing id out of the command: {}", error)),
        }
    }
}
