use clap::{Parser, Subcommand};

mod all;
mod common;
mod contiguous;
mod continuous_loop;
mod new_only;

#[derive(Parser, Debug)]
#[command(name = "mina-indexer-block-util", author, about, long_about = Some("
Download Mina blocks from GCP buckets with ease!"))]
struct Entrypoint {
    /// Only download the new blocks absent from your block dir
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Download all blocks
    All(all::AllArgs),
    /// Download a contiguous collection blocks
    Contiguous(contiguous::ContiguousArgs),
    /// Only download the most recent blocks absent from your block dir
    NewOnly(new_only::NewArgs),
    /// Run the block fetcher in a continuous loop
    Loop(continuous_loop::LoopArgs),
}

fn main() -> anyhow::Result<()> {
    env_logger::init();

    // dispatch appropriate handler
    match Entrypoint::parse().command {
        Command::All(args) => all::main(args),
        Command::Contiguous(args) => contiguous::main(args),
        Command::NewOnly(args) => new_only::main(args),
        Command::Loop(args) => continuous_loop::main(args),
    }
}
