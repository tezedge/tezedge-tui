# Tezedge Terminal User Interface (WIP)

## Prerequisites

- running tezedge node (preferably on default ports for now)
- The node should be built from branch `mempool-fix-over-prechecking` <- TODO

## Build and run

To connect to the default localhost node with default port:

```
cargo run --release
```

To connect to a specific node (Note: this handling will be most likely changed to options, not args):

```
cargo run --release -- http://develop.dev.tezedge.com:18732/ ws://develop.dev.tezedge.com:4927/
```

## Usage

- \'F1\' - Switch to synchronization screen
- \'F2\' - Switch to mempool/endorsements screen
- \'F3\' - Switch to the statistics screen

- \'q\' - quit the application
- \'j\' - rotate sort order to the left
- \'k\' - rotate sort order to the right
- \'d\' - toggle delta values

- \'arrow down\' - Move down in widgets (tables, lists, etc...)
- \'arrow up\' - Move up in widgets (tables, lists, etc...)

- \'Tab\' - Rotate widget focus on the current screen