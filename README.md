# mini-redis

## Overview
This is an implementation of a mini Redis server as a code challenge. The code consists of modules for handling client connections, defining commands, managing a simple in-memory cache, and utility functions.

## How to Run
1. Clone the repository.
2. Navigate to the project directory.
3. Run `cargo build` to build the project.
4. Run the resulting binary with `cargo run`.

## Code Structure
- Listens for incoming TCP connections on `127.0.0.1:6379`.

## How to Extend
- Add new commands: To extend the functionality, add new command handlers in the `Commands` module.
- Enhance cache features: Modify the `Cache` module to support additional features like eviction policies.

## Testing
- Unit tests are provided for the `Cache` and `Commands` modules.
- Run tests using `cargo test`.

## Sample Commands
- `ping`: Responds with "PONG".
- `echo <message>`: Responds with the input message.
- `set <key> <value> [EX <expiry>]`: Sets a key-value pair with an optional expiration time.
- `get <key>`: Retrieves the value associated with the specified key.
