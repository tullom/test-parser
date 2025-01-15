use clap::Parser;
use log::{error, info, trace};
use std::io::{self, BufRead};
use std::process::ExitCode;

fn main() -> ExitCode {
    env_logger::init();

    let args = Args::parse();
    info!("Using test success trigger message as {}", args.success_msg);
    info!("Using test failure trigger message as {}", args.failure_msg);

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.expect("Failed to read line from stdin");
        trace!("{}", line);
        if line.contains(&args.success_msg) {
            println!("Test passed");
            return ExitCode::SUCCESS;
        } else if line.contains(&args.failure_msg) {
            error!("Test failed");
            return ExitCode::FAILURE;
        }
    }
    error!("No termination sequence found");
    ExitCode::FAILURE
}

/// Testing output parser
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Test success trigger message
    #[arg(short, long)]
    success_msg: String,

    /// Test failure trigger message
    #[arg(short, long)]
    failure_msg: String,
}
