use crate::common::check_gsutil;
use clap::Parser;
use fs::check_dir;
use log::error;
use std::{
    path::PathBuf,
    process::{Command, Stdio},
};

#[derive(Parser, Debug, Clone)]
pub struct AllArgs {
    /// Directory to dump blocks into
    #[arg(short, long, default_value = concat!(env!("HOME"), "/.mina-indexer-contiguous-blocks"))]
    blocks_dir: PathBuf,
    /// Name of Mina network
    #[arg(short, long, default_value = "mainnet")]
    network: String,
    /// Name of GCP bucket
    #[arg(long, default_value = "mina_network_block_data")]
    bucket: String,
}

pub fn main(args: AllArgs) -> anyhow::Result<()> {
    let blocks_dir = args.blocks_dir;
    let network = args.network;
    let bucket = args.bucket;

    check_dir(&blocks_dir);
    check_gsutil();

    let query = format!("gs://{bucket}/{network}-*-*.json");
    let mut gsutil_cmd = Command::new("gsutil")
        .arg("-m")
        .arg("cp")
        .arg("-n")
        .arg(&query)
        .arg(blocks_dir)
        .stdout(Stdio::piped())
        .spawn()?;

    match gsutil_cmd.wait() {
        Ok(_) => Ok(()),
        Err(err) => {
            error!("{err}");
            Err(anyhow::Error::msg(err))
        }
    }
}
