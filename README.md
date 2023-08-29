# Iggy - Mina Block Utility Tool

Keep your Mina block data up to date!

This tool is intended to support indexing of the [Mina blockchain](https://github.com/MinaProtocol/mina) via the [mina-indexer](https://github.com/Granola-Team/mina-indexer), but can be used by anyone who wants to download Mina blocks from a GCP bucket via `gsutil`.

> Note: you must have [`gsutil`](https://cloud.google.com/storage/docs/gsutil_install) installed!

## Quick start

Clone the repo

```sh
git clone git@github.com:Granola-Tream/iggy.git
```

To make your life simpler, you may want to create an alias

```txt
alias iggy="RUST_LOG=info $IGGY_HOME/target/release/mina-indexer-block-util"
```

## Use cases

### I want all the blocks

Simple

```sh
RUST_LOG=info cargo run --release --bin mina-indexer-block-util -- all -b /path/to/blocks/dir
```

For more options, see

```sh
cargo run --bin mina-indexer-block-util -- all --help
```

### I just need a contiguous collection of blocks

To get all blocks starting from length `2` and going to `101` and see `Info` level logs, do

```sh
RUST_LOG=info cargo run --release --bin mina-indexer-block-util -- contiguous -b /path/to/blocks/dir
```

For more options, see

```sh
cargo run --bin mina-indexer-block-util -- contiguous --help
```

### I need to sync my local copy of Mina blocks 

To get up-to-date with `mainnet` blocks from the o1-labs [`mina_network_block_data` bucket](https://console.cloud.google.com/storage/browser/mina_network_block_data) and see `Info` level logs, do

```sh
RUST_LOG=info cargo run --release --bin mina-indexer-block-util -- new-only -b /path/to/blocks/dir
```

For more options, see

```sh
cargo run --bin mina-indexer-block-util -- new-only --help
```

### I'm running an indexer and need my local copy of Mina blocks to perpetually update

This is intended to be run after obtaining a nearly up-to-date collection of Mina blocks. To perpetually stay synced with the o1-labs GCP bucket and see `Info` level logs, do

```sh
RUST_LOG=info cargo run --release --bin mina-indexer-block-util -- loop -b /path/to/blocks/dir
```

For more options, see

```sh
cargo run --bin mina-indexer-block-util -- new-only --help
```
