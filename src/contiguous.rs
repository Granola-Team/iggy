use crate::common::check_gsutil;
use clap::Parser;
use fs::{check_dir, check_file};
use log::info;
use std::{
    fs::OpenOptions,
    io::prelude::*,
    path::PathBuf,
    process::{Command, Stdio},
};

#[derive(Parser, Debug, Clone)]
pub struct ContiguousArgs {
    /// File to write queries to
    #[arg(short, long, default_value = concat!(env!("HOME"), "/.mina-indexer-contiguous-block-queries"))]
    query_file: PathBuf,
    /// Directory to dump blocks into
    #[arg(short, long, default_value = concat!(env!("HOME"), "/.mina-indexer-contiguous-blocks"))]
    blocks_dir: PathBuf,
    /// Start block blockchain_length
    #[arg(short, long, default_value_t = 2)]
    start: usize,
    /// Number of block lengths to download
    #[arg(short, long, default_value_t = 1000)]
    num: usize,
    /// Name of Mina network
    #[arg(short, long, default_value = "mainnet")]
    network: String,
    /// Name of GCP bucket
    #[arg(long, default_value = "mina_network_block_data")]
    bucket: String,
}

pub fn main(args: ContiguousArgs) -> anyhow::Result<()> {
    let query_file = args.query_file;
    let blocks_dir = args.blocks_dir;
    let start = args.start;
    let num = args.num;
    let network = args.network;
    let bucket = args.bucket;

    check_file(&query_file);
    check_dir(&blocks_dir);
    check_gsutil();

    // write query file to download the desired Mina blocks
    let mut file = OpenOptions::new().append(true).open(query_file.clone())?;
    file.set_len(0)?;

    info!("Writing query file...");
    for height in start..(num + start) {
        writeln!(file, "gs://{bucket}/{network}-{height}-*.json")?;
    }

    // cat query_file | gsutil -m cp -n -I
    let cat_cmd = Command::new("cat")
        .arg(query_file)
        .stdout(Stdio::piped())
        .spawn()?;

    let gsutil_output = Command::new("gsutil")
        .arg("-m")
        .arg("cp")
        .arg("-n")
        .arg("-I")
        .arg(blocks_dir)
        .stdin(Stdio::from(cat_cmd.stdout.unwrap()))
        .output()?;

    // only output successfully copied blocks
    let output = String::from_utf8(gsutil_output.stderr);
    for line in output?.split('\n').filter(|s| s.starts_with("Copying")) {
        println!("{line}");
    }

    Ok(())
}
