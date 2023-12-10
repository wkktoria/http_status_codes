use std::env;

use colored::Colorize;
use http_status_codes::display_info;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        let status_code = &args[1];
        display_info(status_code);
    } else {
        println!("{}", "\nPass status code as an argument!".red());
    }
}
