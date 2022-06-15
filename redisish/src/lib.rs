use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Command {
    Publish(String),
    Retrieve
}

#[derive(Debug, Clone)]
pub struct Error;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid input format")
    }
}

pub fn parse(input: &str) -> Result<Command, Error> {
    if input == "RETRIEVE\n" {
        return Ok(Command::Retrieve);
    }

    if let Some((command, message)) = input.split_once(" ") {
        if command == "PUBLISH" {
            return Ok(Command::Publish(String::from(message)));
        }
    };

    return Err(Error);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accept_publish() {
        let input = "PUBLISH my message\n";
        let command = parse(&input);
        assert!(command.is_ok());
        if let Ok(Command::Publish(message)) = command {
            assert_eq!(message, "my message\n");
        } else {
            assert!(false);
        }
    }

    #[test]
    fn accept_retrieve() {
        let input = "RETRIEVE\n";
        let command = parse(&input);
        assert!(command.is_ok());
        assert_eq!(command.unwrap(), Command::Retrieve);
    }
}
