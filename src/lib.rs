mod ui;
mod compiler;

use std::thread;
use std::error::Error;
use std::fs;
use std::process;
use std::io;
use std::sync::mpsc;

pub fn open_source(file_path: &str) -> Result<String, io::Error>{

    let contents = fs::read_to_string(file_path)?;
    Ok(contents)
}


pub fn run_session(contents: &str) {
    let
        (
order_tx,
order_rx
        ) = mpsc::channel::<compiler::OrderMessage>();
    let
        (
compiler_tx,
compiler_rx
        ) = mpsc::channel::<compiler::CompileSnapShot>();

    let text  = contents.to_string();
    
    let handle = thread::spawn(move || {
        compiler::compile_session(compiler_tx, order_rx, text);
    });

    order_tx.send(compiler::OrderMessage::Exit);
    
    handle.join().unwrap();
}


