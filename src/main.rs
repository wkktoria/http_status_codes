use std::env;

use colored::Colorize;
use http_status_codes::display_info;

fn main() {
    for status_code in env::args().skip(1) {
        display_info(&status_code);
    }
}
