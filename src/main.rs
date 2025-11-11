#![doc = include_str!("../README.md")]

use {clap::Parser, clap_cargo::style::CLAP_STYLING};

#[derive(Parser)]
#[command(about, version, name = "portpicker", max_term_width = 80, styles = CLAP_STYLING)]
struct Cli {
    /// Pick N free unused ports
    #[arg(short, value_name = "N", default_value = "1")]
    n: usize,
}

fn main() {
    let cli = Cli::parse();

    for _ in 0..cli.n {
        println!("{}", portpicker::pick_unused_port().expect("No ports free"));
    }
}
