use std::{env, io};

use colored::Colorize;
use http_status_codes::display_info;

fn main() {
    let args = env::args();
    if args.len() == 1 {
        let mut status_code = String::new();

        println!("\nStatus code:");
        io::stdin()
            .read_line(&mut status_code)
            .expect(format!("{}", "Failed to read status code.".red()).as_str());

        display_info(&status_code);
    } else {
        for status_code in args.skip(1) {
            display_info(&status_code);
        }
    }
}
