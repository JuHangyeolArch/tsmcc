use std::fs;
use egui::{Color32, FontId, FontFamily, TextFormat};
use egui::text::LayoutJob;

use std::sync::mpsc;
use std::thread;

use crate::tsmcc;
use crate::tsmcc::CompileSnapShot;
use crate::tsmcc::SnapShotPhase;
use crate::tsmcc::OrderMessage;
use crate::tsmcc::NodeID;


pub struct MainMenu {
    
    label: String,
    source: String,
    error: Option<String>,
    font_id: FontId,
    state: Option<CompileSnapShot>,
    focus: Option<(usize, SnapShotPhase)>,
    order_tx: mpsc::Sender<OrderMessage>,
    compiler_rx: mpsc::Receiver<CompileSnapShot>,
    handle: thread::JoinHandle<()>,
}

impl Default for MainMenu {
    fn default() -> Self {

        let source = "Hello World!";
        
        let (handle,order_tx,compiler_rx) =
            tsmcc::start_session(source);

        order_tx.send(OrderMessage::Setup);
        
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            source: source.to_owned(),
            error: None,
            font_id: FontId::new(14.0, FontFamily::Monospace),
            order_tx,
            compiler_rx,
            handle,
            state: None,
            focus: None,
        }

    }
}


impl MainMenu {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
            Default::default()
    }
}

impl eframe::App for MainMenu {
    /// Called by the framework to save state before shutdown.

    /// Called each time the UI needs repainting, which may be many times per second.
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::Panel::top("top_panel").show(ui, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::MenuBar::new().ui(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                
                ui.menu_button("File", |ui| {
                    if ui.button("Open").clicked() {
                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter("TSMC코드", &["tsmc"])
                            .pick_file() 
                {
                    match fs::read_to_string(path.display().to_string()) {
                        Ok(content) => {

                            let (handle,order_tx,compiler_rx)
                                = tsmcc::start_session(&content);

                            
                            let old_handle = std::mem::replace(&mut self.handle, handle);                       
                            self.order_tx.send(OrderMessage::Exit);
                            old_handle.join();
                            std::mem::replace(&mut self.order_tx, order_tx);
                            std::mem::replace(&mut self.compiler_rx, compiler_rx);
                            self.order_tx.send(OrderMessage::Setup);
                            self.source = content;
                            self.state = None;
                            self.focus = None;
                        }
                        Err(err) => {
                            self.error = Some(format!("couldn't open file: {}", err));
                        }
                    }
                }
                    }
                    if ui.button("Quit").clicked() {
                        ui.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.add_space(16.0);

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ui, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("TSMC Compiler");

            
            if let Some(snapshot) = self.state.clone() {
                ui.label(format!("current phase: {:#?}",snapshot.phase)); 

                if let Some(err) = snapshot.error {
                    ui.label(format!("error occured: {:#?}",err)); 
                }

            }


            

            if ui.button("Proceed").clicked() {
                self.state = crate::tsmcc::run_session(&self.order_tx,&self.compiler_rx);
            }


            
            ui.separator();

            egui::ScrollArea::vertical()
                .auto_shrink([false; 2]) 
                .show(ui, |ui| {
                 
            
                    egui::CollapsingHeader::new("0. Source")
                        .default_open(true) 
                        .show(ui, |ui| {
                            let mut source  = LayoutJob::default();
                            let mut focus_range = (None,None);

                            if let Some((pos,phase)) = self.focus {

                                focus_range = tsmcc::show_span(&self.state.clone().unwrap(),pos,phase,SnapShotPhase::Lexer);

                            }
                    
                            for (idx,c) in self.source.chars().enumerate() {

                                if let  (Some(start),Some(end))= focus_range {
                                    if idx >= start && idx < end {
                                        source.append(
                                            &c.to_string(), 
                                            0.0, 
                                            TextFormat {
                                                font_id: self.font_id.clone(),
                                                color: Color32::RED,
                                                ..Default::default()
                                            }
                                        );
                                        continue;
                                    }
                                }
                        
                                source.append(
                                    &c.to_string(), 
                                    0.0, 
                                    TextFormat {
                                        font_id: self.font_id.clone(),
                                        color: ui.visuals().text_color(),
                                        ..Default::default()
                                    }
                                );

                            }
            
                            ui.horizontal(|ui| {
                                ui.label(source);
                            });
                        });

                    egui::CollapsingHeader::new("1. Tokens")
                        .default_open(false) 
                        .show(ui, |ui| {
                            let mut focus_range = (None,None);
                            if let Some((pos,phase)) = self.focus {
                                focus_range = tsmcc::show_span(&self.state.clone().unwrap(),pos,phase,SnapShotPhase::Parser);
                            }

                            if let Some(snapshot) = self.state.clone() {

                                for (idx,token) in snapshot.token_list.as_iter().enumerate() {
                            
                                    let t  = format!("{:?}",token.kind());

                                    let button = egui::Button::new(t)
                                        .fill(
                                            if let  (Some(start),Some(end))= focus_range {
                                                if idx >= start && idx < end {
                                                    Color32::RED
                                                } else {
                                                    ui.visuals().text_color()
                                                }
                                        
                                            } else {
                                                ui.visuals().text_color()
                                            });

                                    if ui.add(button).clicked() {
                                        self.focus = Some((idx,SnapShotPhase::Lexer));
                                    }
                            
                                }

                            }

                        });


                    egui::CollapsingHeader::new("2. AST")
                        .default_open(false) 
                        .show(ui, |ui| {
                            let mut focus_range: (Option<usize>,Option<usize>) = (None,None);
                            if let Some((pos,phase)) = self.focus {
                                focus_range = tsmcc::show_span(&self.state.clone().unwrap(),pos,phase,SnapShotPhase::IrGenerator);
                            }

                            if let Some(snapshot) = self.state.clone() {
                                if snapshot.ast.count() > 0 {
                                    self.render_tree(ui,focus_range,snapshot.ast.get_root());
                                }
                            }
                        });

                    egui::CollapsingHeader::new("3. SymbolTable")
                        .default_open(false) 
                        .show(ui, |ui| {
                    
                            if let Some(snapshot) = self.state.clone() {
                                ui.label(format!("{:#?}",snapshot.func_table));
                            }
                        });

            
                    egui::CollapsingHeader::new("4. Intermediate Representation")
                        .default_open(false) 
                        .show(ui, |ui| {
                            let mut focus_range = (None,None);
                            if let Some((pos,phase)) = self.focus {
                                focus_range = tsmcc::show_span(&self.state.clone().unwrap(),pos,phase,SnapShotPhase::CodeGenerator);
                            }

                            if let Some(snapshot) = self.state.clone() {

                                for (idx,line) in snapshot.iloc.as_iter().enumerate() {
                            
                                    let t  = format!("{:#?}",line);
                                    let button = egui::Button::new(t)
                                        .fill(
                                            if let  (Some(start),Some(end))= focus_range {
                                                if idx >= start && idx < end {
                                                    Color32::RED
                                                } else {
                                                    ui.visuals().text_color()
                                                }
                                        
                                            } else {
                                                ui.visuals().text_color()
                                            });

                                    if ui.add(button).clicked() {
                                        self.focus = Some((idx,SnapShotPhase::IrGenerator));
                                    }
                            
                                }

                            }
                        });
            

   

                    egui::CollapsingHeader::new("5. Assembly")
                        .default_open(false) 
                        .show(ui, |ui| {
                            let mut focus_range = (None,None);
                            if let Some((pos,phase)) = self.focus {
                                focus_range = tsmcc::show_span(&self.state.clone().unwrap(),pos,phase,SnapShotPhase::Assembler);
                            }

                            if let Some(snapshot) = self.state.clone() {

                                for (idx,line) in snapshot.assembly.iter().enumerate() {
                            
                                    let t  = line.to_string();

                                    let button = egui::Button::new(t)
                                        .fill(
                                            if let  (Some(start),Some(end))= focus_range {
                                                if idx >= start && idx < end {
                                                    Color32::RED
                                                } else {
                                                    ui.visuals().text_color()
                                                }
                                        
                                            } else {
                                                ui.visuals().text_color()
                                            });

                                    if ui.add(button).clicked() {
                                        self.focus = Some((idx,SnapShotPhase::CodeGenerator));
                                    }
                            
                                }

                            }
                        });
            
                    egui::CollapsingHeader::new("6. Binary")
                        .default_open(false) 
                        .show(ui, |ui| {

                            if let Some(snapshot) = self.state.clone() {

                                for (idx,line) in snapshot.machine_code.as_iter().enumerate() {
                            
                                    let t  = line.to_string();

                                    let button = egui::Button::new(t)
                                        .fill(
                                            ui.visuals().text_color()
                                        );

                                    if ui.add(button).clicked() {
                                        self.focus = Some((idx,SnapShotPhase::Assembler));
                                    }   
                                }
                            }
                        });
                    

                });
                    
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                bottom_label(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn bottom_label(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Beatles Forever");
    });
}


impl MainMenu {
    fn render_tree(
        &mut self,
        ui: &mut egui::Ui,
        focus: (Option<usize>,Option<usize>),
        node_id: NodeID,
    ) {

        
        let cur_node = self
            .state
            .clone()
            .unwrap()
            .ast.get(node_id)
            .unwrap()
            .clone();


        let id = ui.make_persistent_id(node_id.to_usize());

        let collapse_state = egui::collapsing_header::CollapsingState::load(ui.ctx(),id)
            .unwrap_or_else(|| egui::collapsing_header::CollapsingState::load_with_default_open(ui.ctx(), id, false));;

        let header_res = collapse_state.show_header(ui, |ui| {
            ui.horizontal(|ui| {            
                
                let button = egui::Button::new(format!("{:?}",cur_node.clone().get_kind()))
                    .fill(
                        if let  (Some(start),Some(end))= focus {
                            if node_id.to_usize() >= start && node_id.to_usize() < end {
                                Color32::RED
                            } else {
                                ui.visuals().text_color()
                            }
                                        
                        } else {
                            ui.visuals().text_color()
                        });
                if ui.add(button).clicked() {
                    self.focus = Some((node_id.to_usize(),SnapShotPhase::Parser));
                }
                
            });
        });
        header_res.body(|ui| {
            ui.indent(node_id.to_usize(), |ui| {
                if let Some(lchild) = cur_node.get_lchild() {
                    self.render_tree(ui,focus,lchild);
                }
                if let Some(rchild) = cur_node.get_rchild() {
                    self.render_tree(ui,focus,rchild);
                }
            });
        });


    }

}
