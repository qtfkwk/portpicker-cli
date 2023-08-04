#![doc = include_str!("../README.md")]

use clap::Parser;

#[derive(Parser)]
#[command(about, version, name = "portpicker")]
struct Cli {
    /// Pick N free unused ports
    #[arg(short, value_name = "N", default_value = "1")]
    n: usize,
}

fn main() {
    let cli = Cli::parse();

    for _ in 0..cli.n {
        let port = portpicker::pick_unused_port().expect("No ports free");
        println!("{port}");
    }
}
