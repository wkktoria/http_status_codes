use clap::{arg, Parser};
use colored::Colorize;
use http_status_codes::display_info;

#[derive(Parser)]
struct Args {
    #[arg(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
    codes: Vec<String>,
}

fn main() {
    let args = Args::parse();

    if args.codes.len() == 0 {
        println!("\n{}\n", "Supply at least one status code!".yellow());
        return;
    }

    for code in args.codes {
        display_info(&code)
    }
}
