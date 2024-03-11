use crate::cache::Cache;

pub struct Commands {
    cache: Cache<String, String>,
}

impl Commands {
    pub fn new() -> Self {
        Self {
            cache: Cache::new(),
        }
    }

    fn simple_command(&self, message: &str) -> String {
        format!("+{}\r\n", message)
    }

    fn bulk_command(&self, message: &str) -> String {
        format!("${}\r\n{}\r\n", message.len(), message)
    }

    fn extract_commands<'a>(&self, command: &'a str) -> Vec<&'a str> {
        let mut lines = command.lines();
        lines.next();

        lines
            .filter_map(|line| {
                if line.contains('$') {
                    None
                } else {
                    Some(line.trim())
                }
            })
            .collect::<Vec<_>>()
    }

    fn exec_command(&self, commands: Vec<&str>) -> String {
        match commands[0] {
            "ping" => self.simple_command("PONG"),
            "echo" => self.bulk_command(commands[1]),
            _ => panic!("command not found"),
        }
    }

    pub fn handle_command(&self, buffer: &str) -> String {
        let commands = self.extract_commands(buffer);
        self.exec_command(commands)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_command() {
        let commands = Commands::new();
        assert_eq!(commands.simple_command("hello"), "+hello\r\n");
    }

    #[test]
    fn test_bulk_command() {
        let commands = Commands::new();
        assert_eq!(commands.bulk_command("world"), "$5\r\nworld\r\n");
    }

    #[test]
    fn test_extract_commands() {
        let commands = Commands::new();
        let command = "*3\r\n$3\r\nSET\r\n$5\r\nhello\r\n$5\r\nworld\r\n";
        assert_eq!(commands.extract_commands(command), vec!["SET", "hello", "world"]);
    }

    #[test]
    fn test_exec_command_ping() {
        let commands = Commands::new();
        assert_eq!(commands.exec_command(vec!["ping"]), "+PONG\r\n");
    }

    #[test]
    fn test_exec_command_echo() {
        let commands = Commands::new();
        assert_eq!(commands.exec_command(vec!["echo", "hello"]), "$5\r\nhello\r\n");
    }

    #[test]
    #[should_panic]
    fn test_exec_command_unknown_command() {
        let commands = Commands::new();
        commands.exec_command(vec!["unknown"]);
    }
}

