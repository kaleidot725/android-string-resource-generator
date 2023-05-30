use std::{thread::sleep, time};
use log::{info, warn};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path)?;
    println!("file conent: {}", content);

    let x = 42;
    println!("My lucky number is {}.", x);

    let xs = vec![1, 2, 3];
    println!("The list is: {:?}", xs);
    
    let pb = indicatif::ProgressBar::new(100);
    let time = time::Duration::from_millis(10);
    for i in 0..100 {
        sleep(time);
        pb.inc(1);
    }
    pb.finish_with_message("done");

    env_logger::init();
    info!("starting up");
    warn!("oops, nothing implemented!");

    Ok(())
}
