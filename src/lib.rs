mod ui;
mod compiler;

use std::thread;
use std::error::Error;
use std::fs;
use std::process;
use std::io;
use std::sync::mpsc;

use crate::compiler::CompileSnapShot;

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

    
    
    order_tx.send(compiler::OrderMessage::Setup);

    
 

    loop {
        order_tx.send(compiler::OrderMessage::Go);
        
        match compiler_rx.recv() {                
            Ok(snapshot) => {
                if let Some(e) = snapshot.error {
                    println!("{:#?}",e);
                    break;
                } else if matches!(snapshot.phase,compiler::SnapShotPhase::Finished) {
                    println!("done");
                    break;
                } else {
                    let mut input = String::new();
                    //io::stdin()
                     //   .read_line(&mut input)
                      //  .expect("fuck");
                    
                    match snapshot.phase {
                        compiler::SnapShotPhase::Lexer => {println!("{:#?}", snapshot.token_list.get(snapshot.token_list.len() - 1));},
                        compiler::SnapShotPhase::Parser => {println!("{:#?}", snapshot.ast);},
                        compiler::SnapShotPhase::CsAnalyzer => {println!("{:#?}",snapshot.func_table);},
                        compiler::SnapShotPhase::IrGenerator => {println!("{:#?}", snapshot.iloc);},
                        compiler::SnapShotPhase::CodeGenerator => {println!("{:#?}", snapshot.assembly.to_string());},
                        compiler::SnapShotPhase::Assembler => {break; println!("{:#?}", snapshot.machine_code);},
                        compiler::SnapShotPhase::Finished => { println!("done"); break;},
                        compiler::SnapShotPhase::Error => {println!("{:#?}",snapshot.error.unwrap()); break;},
                    }
                }                
            },
            Err(recv_error) => {continue},
        }
    }

    order_tx.send(compiler::OrderMessage::Exit);
    
    handle.join().unwrap();
}


