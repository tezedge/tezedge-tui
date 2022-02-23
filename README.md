# Tezedge Terminal User Interface (WIP)

## Prerequisites

- None

## Build preparation

```
# Run the following in your terminal, then follow the onscreen instructions.
curl https://sh.rustup.rs -sSf | sh
```

```
rustup toolchain install 1.58.1
rustup default 1.58.1
```



## Build and run

To connect to the default localhost node with default port:

```
cargo run --release
```

To connect to our test run with tezedge and baker/endorser:
```
cargo run --release -- --node http://mempool.tezedge.com:18732/ --websocket ws://mempool.tezedge.com:4927/ --baker-address tz1Mkb2MQyHnVybEru6iTgTGQaZikyg4fBhr
```

## Usage

- \'F1\' - Switch to synchronization screen
- \'F2\' - Switch to mempool/endorsements screen
- \'F3\' - Switch to the statistics screen
- \'F4\' - Switch to the baking screen

- \'F10\' - quit the application
- \'s\' - sort selected column in table
- \'d\' - toggle delta values

- \'arrow down\' - Move down in widgets (tables, lists, etc...)
- \'arrow up\' - Move up in widgets (tables, lists, etc...)
- \'arrow left\' - Move left in widgets (tables, lists, etc...)
- \'arrow right\' - Move right in widgets (tables, lists, etc...)

- \'Tab\' - Rotate widget focus on the current screen