# dcli
### console-plain-configurations

## Setup
The `console-plain-configurations` is the client side to `rest-api-configurations`. Make sure to run the server on another thread.
```
cargo run-rest-api
```

## Example usage
```
cargo run-console -- task create -d "My task description"
cargo run-console -- task view
cargo run-console -- task view -p 4
```
