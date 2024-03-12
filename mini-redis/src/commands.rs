use crate::{cache::Cache, utils};
use std::time::Duration;

pub struct Commands {
    cache: Cache<String, String>,
    default_ttl: Duration,
}

impl Commands {
    pub fn new(ttl: u64) -> Self {
        Self {
            cache: Cache::new(),
            default_ttl: Duration::from_secs(ttl),
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

    fn exec_command(&mut self, commands: Vec<&str>) -> String {
        match commands[0] {
            "ping" => self.simple_command("PONG"),
            "echo" => self.bulk_command(commands[1]),
            "set" => self.set(
                commands[1].to_owned(),
                commands[2].to_owned(),
                utils::parse_to_duration(commands.get(3).cloned()),
            ),
            "get" => self.get(commands[1].to_owned()),
            _ => panic!("command not found"),
        }
    }

    fn get(&mut self, key: String) -> String {
        match self.cache.get(&key) {
            Some(value) => self.simple_command(&value),
            None => self.simple_command("Not found."),
        }
    }

    fn set(&mut self, key: String, value: String, ttl: Option<Duration>) -> String {
        match ttl {
            None => self.cache.set(key, value, self.default_ttl),
            _ => self.cache.set(key, value, ttl.unwrap()),
        };

        self.simple_command("Ok")
    }

    pub fn handle_command(&mut self, buffer: &str) -> String {
        let commands = self.extract_commands(buffer);
        self.exec_command(commands)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exec_command_get_found() {
        let mut commands = Commands::new(5);
        commands.set("key".to_owned(), "value".to_owned(), None);
        assert_eq!(commands.exec_command(vec!["get", "key"]), "+value\r\n");
    }

    #[test]
    fn test_exec_command_get_not_found() {
        let mut commands = Commands::new(5);
        assert_eq!(
            commands.exec_command(vec!["get", "nonexistent"]),
            "+Not found.\r\n"
        );
    }

    #[test]
    fn test_exec_command_set_no_ttl() {
        let mut commands = Commands::new(1);
        assert_eq!(
            commands.exec_command(vec!["set", "key", "value"]),
            "+Ok\r\n"
        );
        assert_eq!(commands.exec_command(vec!["get", "key"]), "+value\r\n");
    }

    #[test]
    fn test_exec_command_set_with_ttl() {
        let mut commands = Commands::new(1);
        assert_eq!(
            commands.exec_command(vec!["set", "key", "value", "EX", "10"]),
            "+Ok\r\n"
        );
        assert_eq!(commands.exec_command(vec!["get", "key"]), "+value\r\n");

        std::thread::sleep(Duration::from_secs(3));
        assert_eq!(commands.exec_command(vec!["get", "key"]), "+Not found.\r\n");
    }

    #[test]
    fn test_simple_command() {
        let commands = Commands::new(5);
        assert_eq!(commands.simple_command("hello"), "+hello\r\n");
    }

    #[test]
    fn test_bulk_command() {
        let commands = Commands::new(5);
        assert_eq!(commands.bulk_command("world"), "$5\r\nworld\r\n");
    }

    #[test]
    fn test_extract_commands() {
        let commands = Commands::new(5);
        let command = "*3\r\n$3\r\nSET\r\n$5\r\nhello\r\n$5\r\nworld\r\n";
        assert_eq!(
            commands.extract_commands(command),
            vec!["SET", "hello", "world"]
        );
    }

    #[test]
    fn test_exec_command_ping() {
        let mut commands = Commands::new(5);
        assert_eq!(commands.exec_command(vec!["ping"]), "+PONG\r\n");
    }

    #[test]
    fn test_exec_command_echo() {
        let mut commands = Commands::new(5);
        assert_eq!(
            commands.exec_command(vec!["echo", "hello"]),
            "$5\r\nhello\r\n"
        );
    }

    #[test]
    #[should_panic]
    fn test_exec_command_unknown_command() {
        let mut commands = Commands::new(5);
        commands.exec_command(vec!["unknown"]);
    }
}
