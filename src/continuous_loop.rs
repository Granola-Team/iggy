use crate::common::check_gsutil;
use blockchain::length_from_path;
use clap::Parser;
use fs::{check_dir, check_file};
use glob::glob;
use log::{debug, info};
use std::{
    fs::File,
    io::prelude::*,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    time::Duration,
};

#[derive(Parser, Debug, Clone)]
pub struct LoopArgs {
    /// Directory to dump blocks into
    #[arg(short, long, default_value = concat!(env!("HOME"), "/.mina-indexer-loop-blocks"))]
    blocks_dir: PathBuf,
    /// How often to query for new blocks (in sec)
    #[arg(short, long, default_value_t = 10)]
    frequency: u64,
    /// File to write queries to
    #[arg(short, long, default_value = concat!(env!("HOME"), "/.mina-indexer-loop-block-queries"))]
    query_file: PathBuf,
    /// Name of Mina network
    #[arg(short, long, default_value = "mainnet")]
    network: String,
    /// Name of GCP bucket
    #[arg(long, default_value = "mina_network_block_data")]
    bucket: String,
    /// Number of previous block_lengths to query each time
    #[arg(long, default_value_t = 5)]
    buffer: u32,
    /// Number of block_lengths above current max to query each time before entering the maintenance loop
    #[arg(long, default_value_t = 100)]
    additional: u32,
}

pub fn main(args: LoopArgs) -> anyhow::Result<()> {
    let blocks_dir = args.blocks_dir;
    let frequency = args.frequency;
    let network = args.network;
    let bucket = args.bucket;
    let buffer = args.buffer;
    let query_file_path = args.query_file;
    let additional = args.additional;

    check_file(&query_file_path);
    check_dir(&blocks_dir);
    check_gsutil();

    info!("Doing the initial catchup...");
    let sleep_duration = Duration::new(frequency, 0);

    // before entering the maintenance loop, we grab blocks until we find a length that doesn't exist
    let mut output = read_current_blocks_and_query(
        &blocks_dir,
        &query_file_path,
        &network,
        &bucket,
        buffer,
        additional,
    )?;

    while !output
        .split('\n')
        .any(|s| s.starts_with("CommandException"))
    {
        output = read_current_blocks_and_query(
            &blocks_dir,
            &query_file_path,
            &network,
            &bucket,
            buffer,
            additional,
        )?;
    }

    std::thread::sleep(sleep_duration);
    info!("Entering maintenance loop...");

    // maintenance loop
    loop {
        read_current_blocks_and_query(
            &blocks_dir,
            &query_file_path,
            &network,
            &bucket,
            buffer,
            frequency as u32 / 3 + 1,
        )?;

        std::thread::sleep(sleep_duration);
    }
}

fn read_current_blocks_and_query(
    blocks_dir: &Path,
    query_file_path: &Path,
    network: &str,
    bucket: &str,
    buffer: u32,
    additional: u32,
) -> anyhow::Result<String> {
    info!("Reading {network} blocks in {}...", blocks_dir.display());
    let mut our_block_paths: Vec<PathBuf> =
        glob(&format!("{}/{network}-*-*.json", blocks_dir.display()))
            .unwrap()
            .filter_map(|p| p.ok())
            .collect();
    our_block_paths.sort_by_key(|x| length_from_path(x).unwrap_or(0));

    let max_network_length = our_block_paths
        .last()
        .map_or(0, |path| length_from_path(path).unwrap());

    info!("Max length of {network} blocks: {max_network_length}");
    debug!("Writing query file: {}", query_file_path.display());

    let mut query_file = File::create(query_file_path).unwrap();
    query_file.set_len(0)?;

    let start = 2.max(max_network_length.saturating_sub(buffer));
    let end = max_network_length + additional;

    for length in start..=end {
        writeln!(query_file, "gs://{bucket}/{network}-{length}-*.json")?;
    }

    let cat_cmd = Command::new("cat")
        .arg(query_file_path)
        .stdout(Stdio::piped())
        .spawn()?;

    let gsutil_cmd = Command::new("gsutil")
        .arg("-m")
        .arg("cp")
        .arg("-n")
        .arg("-I")
        .arg(blocks_dir)
        .stdin(Stdio::from(cat_cmd.stdout.unwrap()))
        .output()?;

    let output = String::from_utf8(gsutil_cmd.stderr.clone()).map_err(anyhow::Error::new);
    for line in output?.split('\n').filter(|s| s.starts_with("Copying")) {
        println!("{line}");
    }

    String::from_utf8(gsutil_cmd.stderr).map_err(anyhow::Error::new)
}
