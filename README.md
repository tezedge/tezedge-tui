# Tezedge Terminal User Interface

## Prerequisites installation

If you have already built the TezEdge node from sources, you can skip these steps

1. Install **Rust** command _(We recommend installing Rust through rustup.)_
    ```
    # Run the following in your terminal, then follow the onscreen instructions.
    curl https://sh.rustup.rs -sSf | sh
    ```
2. Install **Rust toolchain** _(Our releases are built with 1.58.1.)_
    ```
    rustup toolchain install 1.58.1
    rustup default 1.58.1
    ```

3. Install **required OS libs**

    ```
    sudo apt install openssl libssl-dev build-essential pkg-config
    ```


## Build and run

First thing should be reading the included help to help undirstand the options the terminal UI can run with
```
cargo run --release -- -h
```

To connect to the default localhost node with default port in the default mode (without baker-address specified):

```
cargo run --release
```

If you are running a baking and endorsing stack with the TezEdge node, you can specify your tezos account as baker address
```
cargo run --release -- --baker-address tz1iFzuDWaP7eKbdBgmcdtz5FAJzDuQqcucm
```

To connect to our test run with TezEdge and baker/endorser:
```
cargo run --release -- --node http://mempool.tezedge.com:18732/ --websocket ws://mempool.tezedge.com:4927/ --baker-address tz1Mkb2MQyHnVybEru6iTgTGQaZikyg4fBhr
```

## Shortcuts

- \'F1\' - Switch to mempool/endorsements screen
- \'F2\' - Switch to the baking screen

- \'F10\' - quit the application
- \'s\' - sort selected column in table
- \'d\' - toggle delta values

- \'arrow down\' - Move down in widgets (tables, lists, etc...)
- \'arrow up\' - Move up in widgets (tables, lists, etc...)
- \'arrow left\' - Move left in widgets (tables, lists, etc...)
- \'arrow right\' - Move right in widgets (tables, lists, etc...)

- \'Tab\' - Rotate widget focus on the current screen