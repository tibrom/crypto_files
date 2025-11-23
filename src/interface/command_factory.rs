use crate::domain::models::Command;

pub struct CommandFactory;

impl CommandFactory {
    pub fn from_args(args: &[String]) -> Result<Command, String> {
        if args.len() != 3 {
            return Err("Использование: <command> <path>".into());
        }

        let command_name = args[1].as_str();
        let path = args[2].clone();

        match command_name {
            "prepare" => Ok(Command::Prepare(path)),
            "read" => Ok(Command::Read(path)),
            "decrypt" => Ok(Command::Decrypt(path)),
            other => Err(format!("Неизвестная команда: {other}")),
        }
    }
}
