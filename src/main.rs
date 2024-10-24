#![doc = include_str!("../README.md")]

use clap::{builder::Styles, Parser};

const STYLES: Styles = Styles::styled()
    .header(clap_cargo::style::HEADER)
    .usage(clap_cargo::style::USAGE)
    .literal(clap_cargo::style::LITERAL)
    .placeholder(clap_cargo::style::PLACEHOLDER)
    .error(clap_cargo::style::ERROR)
    .valid(clap_cargo::style::VALID)
    .invalid(clap_cargo::style::INVALID);

#[derive(Parser)]
#[command(about, version, name = "portpicker", max_term_width = 80, styles = STYLES)]
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
