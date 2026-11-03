
mod compiler;

use std::thread;
use std::error::Error;
use std::fs;
use std::process;
use std::io;
use std::sync::mpsc;

pub use crate::compiler::CompileSnapShot;
use crate::compiler::ProducedBy;
pub use crate::compiler::SnapShotPhase;
pub use crate::compiler::OrderMessage;
pub use crate::compiler::SpanInfo;
pub use crate::compiler::ast::NodeID;

pub fn start_session(contents: &str) -> (thread::JoinHandle<()>, mpsc::Sender<OrderMessage>, mpsc::Receiver<CompileSnapShot>) {
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

    (handle, order_tx, compiler_rx)

    

}

pub fn run_session(
    order_tx: & mpsc::Sender<OrderMessage>,
    compiler_rx: & mpsc::Receiver<CompileSnapShot>
) -> Option<CompileSnapShot> {
     
    order_tx.send(compiler::OrderMessage::Go);
        
    match compiler_rx.recv() {                
        Ok(snapshot) => {Some(snapshot)}
        Err(_) => {None},
    }
}


pub fn end_session(handle: thread::JoinHandle<()>, order_tx: &mpsc::Sender<OrderMessage>) {
    order_tx.send(compiler::OrderMessage::Exit);    
    handle.join().unwrap();

}

pub fn show_span(
    snapshot: &CompileSnapShot,
    target: usize,
    phase: SnapShotPhase,
    dest_phase: SnapShotPhase,
) -> (Option<usize>,Option<usize>) {

    match phase {
        SnapShotPhase::Lexer => {
            match dest_phase {
                SnapShotPhase::Lexer => {
                    let span = snapshot
                        .token_list
                        .get(target)
                        .unwrap()
                        .produced_by()
                        .unwrap();
                    (span.get_start(), span.get_end())          
                },
                SnapShotPhase::Parser => (Some(target), Some(target + 1)),
                _ => (None,None),
            }
        },
        SnapShotPhase::Parser => {
            match dest_phase {
                SnapShotPhase::Lexer => {
                    let mut first: Option<usize> = None;
                    let mut last: Option<usize> = None;
                    let span = snapshot
                        .ast
                        .get(NodeID::new(target))
                        .unwrap()
                        .produced_by()
                        .unwrap();

                    if let (Some(start),Some(end)) = (span.get_start(),span.get_end()) {
                        for i in start..end {
                            if let (Some(cur_start),Some(cur_end)) = show_span(
                                snapshot,i,
                                SnapShotPhase::Lexer,
                                SnapShotPhase::Lexer
                            ) {
                                if let (Some(prev_start), Some(prev_end)) = (first, last) {
                                    if prev_end < cur_end {
                                        last = Some(cur_end);
                                    }
                                } else {
                                    (first, last) = (Some(cur_start),Some(cur_end));
                                }
                            }
                        }                          
                    }
                    
                    (first,last)
                },
                SnapShotPhase::Parser => {
                    let span = snapshot
                        .ast
                        .get(NodeID::new(target))
                        .unwrap()
                        .produced_by()
                        .unwrap();
                    (span.get_start(), span.get_end())
                },
                SnapShotPhase::IrGenerator => (Some(target), Some(target + 1)),
                _ => (None,None),
            }
        },
        SnapShotPhase::CsAnalyzer => {
            return (None,None);
        },
        SnapShotPhase::IrGenerator => {
            match dest_phase {
                SnapShotPhase::Lexer => {
                    let mut first: Option<usize> = None;
                    let mut last: Option<usize> = None;
                    let span = snapshot
                        .iloc
                        .get_by_global(target)
                        .unwrap()
                        .produced_by()
                        .unwrap();

                    if let (Some(start),Some(end)) = (span.get_start(),span.get_end()) {
                        for i in start..end {
                            if let (Some(cur_start),Some(cur_end)) = show_span(
                                snapshot,i,
                                SnapShotPhase::Parser,
                                SnapShotPhase::Lexer
                            ) {
                                if let (Some(prev_start), Some(prev_end)) = (first, last) {
                                    if prev_end < cur_end {
                                        last = Some(cur_end);
                                    }
                                } else {
                                    (first, last) = (Some(cur_start),Some(cur_end));
                                }
                            }
                        }                          
                    }
                    (first,last)
                },

                SnapShotPhase::Parser => {
                    let mut first: Option<usize> = None;
                    let mut last: Option<usize> = None;
                    let span = snapshot
                        .iloc
                        .get_by_global(target)
                        .unwrap()
                        .produced_by()
                        .unwrap();

                    if let (Some(start),Some(end)) = (span.get_start(),span.get_end()) {
                        for i in start..end {
                            if let (Some(cur_start),Some(cur_end)) = show_span(
                                snapshot,i,
                                SnapShotPhase::Parser,
                                SnapShotPhase::Parser
                            ) {
                                if let (Some(prev_start), Some(prev_end)) = (first, last) {
                                    if prev_end < cur_end {
                                        last = Some(cur_end);
                                    }
                                } else {
                                    (first, last) = (Some(cur_start),Some(cur_end));
                                }
                            }
                        }                          
                    }
                    
                    (first,last)

                }
                
                SnapShotPhase::IrGenerator => {
                    let span = snapshot
                        .iloc
                        .get_by_global(target)
                        .unwrap()
                        .produced_by()
                        .unwrap();
                    (span.get_start(), span.get_end())
                },
                SnapShotPhase::CodeGenerator => (Some(target), Some(target + 1)),
                _ => (None,None),
            }

        },
        SnapShotPhase::CodeGenerator => {
            match dest_phase {
                SnapShotPhase::Lexer => {
                    let mut first: Option<usize> = None;
                    let mut last: Option<usize> = None;
                    let span = snapshot
                        .assembly
                        .get_line(target)
                        .unwrap()
                        .produced_by()
                        .unwrap();

                    
                    if let (Some(start),Some(end)) = (span.get_start(),span.get_end()) {
                        for i in start..end {
                            if let (Some(cur_start),Some(cur_end)) = show_span(
                                snapshot,i,
                                SnapShotPhase::IrGenerator,
                                SnapShotPhase::Lexer
                            ) {
                                if let (Some(prev_start), Some(prev_end)) = (first, last) {
                                    if prev_end < cur_end {
                                        last = Some(cur_end);
                                    }
                                } else {
                                    (first, last) = (Some(cur_start),Some(cur_end));
                                }
                            }
                        }                          
                    }
                    
                    (first,last)
                },

                SnapShotPhase::Parser => {
                    let mut first: Option<usize> = None;
                    let mut last: Option<usize> = None;
                    let span = snapshot
                        .assembly
                        .get_line(target)
                        .unwrap()
                        .produced_by()
                        .unwrap();
                    
                    if let (Some(start),Some(end)) = (span.get_start(),span.get_end()) {
                        for i in start..end {
                            if let (Some(cur_start),Some(cur_end)) = show_span(
                                snapshot,i,
                                SnapShotPhase::IrGenerator,
                                SnapShotPhase::Parser
                            ) {
                                if let (Some(prev_start), Some(prev_end)) = (first, last) {
                                    if prev_end < cur_end {
                                        last = Some(cur_end);
                                    }
                                } else {
                                    (first, last) = (Some(cur_start),Some(cur_end));
                                }
                            }
                        }                          
                    }
                    
                    (first,last)

                },

                SnapShotPhase::IrGenerator => {
                    let mut first: Option<usize> = None;
                    let mut last: Option<usize> = None;
                    let span = snapshot
                        .assembly
                        .get_line(target)
                        .unwrap()
                        .produced_by()
                        .unwrap();
                    
                    if let (Some(start),Some(end)) = (span.get_start(),span.get_end()) {
                        for i in start..end {
                            if let (Some(cur_start),Some(cur_end)) = show_span(
                                snapshot,i,
                                SnapShotPhase::IrGenerator,
                                SnapShotPhase::IrGenerator
                            ) {
                                if let (Some(prev_start), Some(prev_end)) = (first, last) {
                                    if prev_end < cur_end {
                                        last = Some(cur_end);
                                    }
                                } else {
                                    (first, last) = (Some(cur_start),Some(cur_end));
                                }
                            }
                        }                          
                    }
                    
                    (first,last)
                },

                SnapShotPhase::CodeGenerator => {
                    let span = snapshot
                        .assembly
                        .get_line(target)
                        .unwrap()
                        .produced_by()
                        .unwrap();
                    (span.get_start(), span.get_end())
                },
                SnapShotPhase::Assembler => (Some(target), Some(target + 1)),
                _ => (None,None),
            }

        },
        SnapShotPhase::Assembler => {
            match dest_phase {
                SnapShotPhase::Lexer => {
                    let mut first: Option<usize> = None;
                    let mut last: Option<usize> = None;
                    let span = snapshot
                        .machine_code
                        .get(target)
                        .unwrap()
                        .produced_by()
                        .unwrap();

                    
                    if let (Some(start),Some(end)) = (span.get_start(),span.get_end()) {
                        for i in start..end {
                            if let (Some(cur_start),Some(cur_end)) = show_span(
                                snapshot,i,
                                SnapShotPhase::CodeGenerator,
                                SnapShotPhase::Lexer
                            ) {
                                if let (Some(prev_start), Some(prev_end)) = (first, last) {
                                    if prev_end < cur_end {
                                        last = Some(cur_end);
                                    }
                                } else {
                                    (first, last) = (Some(cur_start),Some(cur_end));
                                }
                            }
                        }                          
                    }
                    
                    (first,last)
                },

                SnapShotPhase::Parser => {
                    let mut first: Option<usize> = None;
                    let mut last: Option<usize> = None;
                    let span = snapshot
                        .machine_code
                        .get(target)
                        .unwrap()
                        .produced_by()
                        .unwrap();
                    
                    if let (Some(start),Some(end)) = (span.get_start(),span.get_end()) {
                        for i in start..end {
                            if let (Some(cur_start),Some(cur_end)) = show_span(
                                snapshot,i,
                                SnapShotPhase::CodeGenerator,
                                SnapShotPhase::Parser
                            ) {
                                if let (Some(prev_start), Some(prev_end)) = (first, last) {
                                    if prev_end < cur_end {
                                        last = Some(cur_end);
                                    }
                                } else {
                                    (first, last) = (Some(cur_start),Some(cur_end));
                                }
                            }
                        }                          
                    }
                    
                    (first,last)

                },

                SnapShotPhase::IrGenerator => {
                    let mut first: Option<usize> = None;
                    let mut last: Option<usize> = None;
                    let span = snapshot
                        .machine_code
                        .get(target)
                        .unwrap()
                        .produced_by()
                        .unwrap();
                    
                    if let (Some(start),Some(end)) = (span.get_start(),span.get_end()) {
                        for i in start..end {
                            if let (Some(cur_start),Some(cur_end)) = show_span(
                                snapshot,i,
                                SnapShotPhase::CodeGenerator,
                                SnapShotPhase::IrGenerator
                            ) {
                                if let (Some(prev_start), Some(prev_end)) = (first, last) {
                                    if prev_end < cur_end {
                                        last = Some(cur_end);
                                    }
                                } else {
                                    (first, last) = (Some(cur_start),Some(cur_end));
                                }
                            }
                        }                          
                    }
                    
                    (first,last)
                },

                SnapShotPhase::CodeGenerator => {
                    let mut first: Option<usize> = None;
                    let mut last: Option<usize> = None;
                    let span = snapshot
                        .machine_code
                        .get(target)
                        .unwrap()
                        .produced_by()
                        .unwrap();
                    
                    if let (Some(start),Some(end)) = (span.get_start(),span.get_end()) {
                        for i in start..end {
                            if let (Some(cur_start),Some(cur_end)) = show_span(
                                snapshot,i,
                                SnapShotPhase::CodeGenerator,
                                SnapShotPhase::CodeGenerator
                            ) {
                                if let (Some(prev_start), Some(prev_end)) = (first, last) {
                                    if prev_end < cur_end {
                                        last = Some(cur_end);
                                    }
                                } else {
                                    (first, last) = (Some(cur_start),Some(cur_end));
                                }
                            }
                        }                          
                    }
                    
                    (first,last)

                },

                SnapShotPhase::Assembler => {
                    let span = snapshot
                        .machine_code
                        .get(target)
                        .unwrap()
                        .produced_by()
                        .unwrap();
                    (span.get_start(), span.get_end())
                },

                SnapShotPhase::Finished => (Some(target), Some(target + 1)),
                _ => (None,None),

            }

        },        
        _ => {return (None,None);}
    }


}

