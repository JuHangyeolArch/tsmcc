use tsmcc;

use std::error::Error;
use std::fs;
use std::process;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let contents = tsmcc::open_source(file_path).unwrap();

    println!("{contents}");

    tsmcc::run_session(&contents);

}
