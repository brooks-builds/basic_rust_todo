pub enum Command {
    ListAll,
    Add(String),
    Done(usize),
    Remove(usize),
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
            "done" => {
                let id = match arguments[1].parse::<usize>() {
                    Ok(id) => id,
                    Err(error) => {
                        return Err(format!("Error parsing id out of the command: {}", error))
                    }
                };
                Ok(Command::Done(id))
            }
            "delete" => {
                let id = match arguments[1].parse() {
                    Ok(id) => id,
                    Err(error) => {
                        return Err(format!("error parsing id out of the command: {}", error))
                    }
                };
                Ok(Command::Remove(id))
            }
            _ => Ok(Command::ListAll),
        }
    }
}
