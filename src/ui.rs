
use egui::{Color32, FontId, FontFamily, TextFormat, Ui, WidgetText};
use egui::text::LayoutJob;
use egui_tiles::{Behavior, TileId, Tiles, Tree, UiResponse};

use std::sync::mpsc;
use std::thread;
use std::fs;
use std::io::Write;

use crate::tsmcc;
use crate::tsmcc::CompileSnapShot;
use crate::tsmcc::SnapShotPhase;
use crate::tsmcc::OrderMessage;
use crate::tsmcc::NodeID;

#[derive(Clone, Debug, PartialEq)]
enum Pane {
    TextEditor,
    Controller,
    Output(SnapShotPhase),
    Settings,
    Manual,    
}


impl Behavior<Pane> for MainMenu {

    fn pane_ui(
        &mut self,
        ui: &mut egui::Ui,
        tile_id: TileId,
        pane: &mut Pane,
    ) -> UiResponse {

        match pane {
            Pane::TextEditor => {
                self.render_text_editor(ui);
            },
            Pane::Controller => {
                self.render_controller(ui);
            },
            Pane::Output(phase) => {
                match phase {
                    SnapShotPhase::Lexer => {
                        self.render_source(ui);
                    },
                    SnapShotPhase::Parser => {
                        self.render_tokens(ui);
                    },
                    SnapShotPhase::IrGenerator => {
                        self.render_ast(ui);
                    },
                    SnapShotPhase::CodeGenerator => {
                        self.render_ir(ui);
                    },
                    SnapShotPhase::Assembler => {
                        self.render_asm(ui);
                    },
                    SnapShotPhase::Finished => {
                        self.render_binary(ui);
                    },
                    SnapShotPhase::Error => {
                        self.render_tokens(ui);
                    }
                    SnapShotPhase::CsAnalyzer => {
                    }
                }
            },
            Pane::Settings => {
                self.render_config(ui);
            },
            Pane::Manual => {
                self.render_manual(ui);
            },
            
        }

        UiResponse::None
    }
    
    fn tab_title_for_pane(&mut self, pane: &Pane) -> egui::WidgetText {
        match pane {
            Pane::TextEditor => "편집기".into(),
            Pane::Controller => "조작기".into(),
            Pane::Output(phase) => {
                match phase {
                    SnapShotPhase::Lexer => {"소스".into()},
                    SnapShotPhase::Parser => {"토큰".into()},
                    SnapShotPhase::IrGenerator => {"AST".into()},
                    SnapShotPhase::CodeGenerator => {"IR".into()},
                    SnapShotPhase::Assembler => {"어셈블리".into()},
                    SnapShotPhase::Finished => {"기계어".into()},
                    SnapShotPhase::Error => {"에러".into()}
                    SnapShotPhase::CsAnalyzer => {"심볼테이블".into()}
                }
            },
            Pane::Settings => "설정".into(),
            Pane::Manual => "메뉴얼".into(),
        }        
    }

    fn simplification_options(&self) -> egui_tiles::SimplificationOptions {
        let mut opts = egui_tiles::SimplificationOptions::default();
        opts.all_panes_must_have_tabs = true;
        
        opts
    }

}

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
    tree: Option<Tree<Pane>>
}

impl Default for MainMenu {
    fn default() -> Self {

        let source = "Hello World!";
        
        let (handle,order_tx,compiler_rx) =
            tsmcc::start_session(source);

        order_tx.send(OrderMessage::Setup);

        let mut tiles = Tiles::default();
        let editor = tiles.insert_pane(Pane::TextEditor);
        let controller = tiles.insert_pane(Pane::Controller);
        let text = tiles.insert_pane(Pane::Output(SnapShotPhase::Lexer));
        let tokens = tiles.insert_pane(Pane::Output(SnapShotPhase::Parser));
        let ast = tiles.insert_pane(Pane::Output(SnapShotPhase::IrGenerator));
        let ir = tiles.insert_pane(Pane::Output(SnapShotPhase::CodeGenerator));
        let asm = tiles.insert_pane(Pane::Output(SnapShotPhase::Assembler));
        let binary = tiles.insert_pane(Pane::Output(SnapShotPhase::Finished));
        let settings = tiles.insert_pane(Pane::Settings);
        let manual = tiles.insert_pane(Pane::Manual);
        
        let right_tabs = tiles.insert_tab_tile(vec![
            editor,
            controller,
            text,
            tokens,
            ast,
            ir,
            asm,
            binary,
            settings,
            manual
        ]);
        let root = tiles.insert_horizontal_tile(vec![right_tabs]);
        
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            source: source.to_owned(),
            error: None,
            font_id: FontId::new(30.0, FontFamily::Monospace),
            order_tx,
            compiler_rx,
            handle,
            state: None,
            focus: None,
            tree: Some(Tree::new(egui::Id::unique("ui_tree"), root, tiles)),
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
                            self.source = content;
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

            if let Some(mut tree) = self.tree.take() {
                tree.ui(self, ui);
                self.tree = Some(tree);
            }
            ui.separator();
                    
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

    fn render_text_editor(
        &mut self,
        ui: &mut egui::Ui,
    ) {
        ui.text_edit_multiline(&mut self.source);
        
        

        let eg1  = egui::RichText::new("example code 1")
            .font(self.font_id.clone());
        let eg2  = egui::RichText::new("example code 2")
            .font(self.font_id.clone());
        let eg3  = egui::RichText::new("example code 3")
            .font(self.font_id.clone());
 

        if ui.button(eg1).clicked() {
                
            self.source = r"
int foo() {
    return 0;
}
".to_string()

        }

            
        if ui.button(eg2).clicked() {
                
            self.source = r"
int foo(int c, int a){
    
    int **ab;
	
    put(10);

    if (1) {
       ;
    }

    return 0;
}

int bar() {
    

    return 0;
}

".to_string()

        }

        if ui.button(eg3).clicked() {

            self.source = r"
int foo(int c, int a){
    
    int a;
    int b;
    a = get();
    b = get();
    put(a+b);

    return 0;
}

".to_string()

        }

                 
        let t  = egui::RichText::new("Connect")
        .font(self.font_id.clone());
        
        
        if ui.button(t).clicked() {
            
            let (handle,order_tx,compiler_rx)
                = tsmcc::start_session(&self.source);
                
                        
            let old_handle = std::mem::replace(&mut self.handle, handle);                       
            self.order_tx.send(OrderMessage::Exit);
            old_handle.join();
            std::mem::replace(&mut self.order_tx, order_tx);
            std::mem::replace(&mut self.compiler_rx, compiler_rx);
            self.order_tx.send(OrderMessage::Setup);
                
            self.state = None;
            self.focus = None;
        }


    }
    
    fn render_controller(
        &mut self,
        ui: &mut egui::Ui,
    ) {
        
        let t  = egui::RichText::new("Proceed")
            .font(self.font_id.clone());

        
        if ui.button(t).clicked() {
                
            match self.state
                .clone()
                .unwrap_or_else(|| {
                    self.state = crate::tsmcc::run_session(&self.order_tx,&self.compiler_rx);
                    self.state.clone().unwrap().clone()
                })
                .phase {
                    SnapShotPhase::Finished => {}
                    SnapShotPhase::Error => {}
                    _ => {
                        let cur_phase = self.state.clone().unwrap().phase;
                        while cur_phase == self.state.clone().unwrap().phase {
                            self.state = crate::tsmcc::run_session(&self.order_tx,&self.compiler_rx);
                        }
                    }
                }

            if SnapShotPhase::CsAnalyzer == self.state.clone().unwrap().phase {
                let cur_phase = self.state.clone().unwrap().phase;
                while cur_phase == self.state.clone().unwrap().phase {
                    self.state = crate::tsmcc::run_session(&self.order_tx,&self.compiler_rx);
                }
            }

        }
                    
        if let Some(snapshot) = self.state.clone() {
            ui.label(egui::RichText::new(format!("current phase: {:#?}",snapshot.phase))
                .font(self.font_id.clone())
            ); 

            if let Some(err) = snapshot.error {
                ui.label(format!("error occured: {:#?}",err)); 
            }

            if snapshot.phase == SnapShotPhase::Finished {

                let t  = egui::RichText::new("기계어 저장ㅋ")
                    .font(self.font_id.clone());
 

                if ui.button(t).clicked() {

                    if let Some(file_path) = rfd::FileDialog::new()
                        .set_title("아웃풋 파일 저장")
                        .add_filter("텍스트 파일", &[""])
                        .save_file() {
                            match fs::File::create(&file_path) {
                            Ok(mut file) => {
                                if let Err(e) = file.write_all(&snapshot.machine_code.by_16word().as_bytes()) {
                                    eprintln!("파일 쓰기 실패: {}", e);
                                } else {
                                    println!("파일이 성공적으로 저장되었습니다!");
                                }
                            }
                            Err(e) => eprintln!("파일 생성 실패: {}", e),

                        }

                

                    }
                }
            }
            

        }
    }

    
    fn render_source(
        &mut self,
        ui: &mut egui::Ui,
    ) {
        egui::ScrollArea::vertical()
            .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysVisible)
            .show(ui, |ui| {
          let mut source  = LayoutJob::default();
                let mut focus_range = (None,None);

                if let Some((pos,phase)) = self.focus {

                    focus_range = tsmcc::show_span(&self.state.clone().unwrap(),pos,phase,SnapShotPhase::Lexer);

                }

                if let Some(snapshot) = self.state.clone() {
                
                    for (idx,c) in snapshot.text.chars().enumerate() {

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
                }
            });
    }
    

    fn render_tokens(
        &mut self,
        ui: &mut egui::Ui,
    ) {
        egui::ScrollArea::vertical()
            .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysVisible)
            .show(ui, |ui| {
                let mut focus_range = (None,None);
                if let Some((pos,phase)) = self.focus {
                    focus_range = tsmcc::show_span(&self.state.clone().unwrap(),pos,phase,SnapShotPhase::Parser);
                }

                if let Some(snapshot) = self.state.clone() {

                    for (idx,token) in snapshot.token_list.as_iter().enumerate() {
                            
                        let t  = egui::RichText::new(format!("{:?}",token.kind()))
                            .font(self.font_id.clone());

                        let button = egui::Button::new(t)
                            .fill(
                                if let  (Some(start),Some(end))= focus_range {
                                    if idx >= start && idx < end {
                                        Color32::RED
                                    } else {
                                        Color32::YELLOW
                                    }
                                        
                                } else {
                                    Color32::YELLOW
                                });

                        if ui.add(button).clicked() {
                            self.focus = Some((idx,SnapShotPhase::Lexer));
                        }
                            
                    }

                }

            });
    }

    fn render_ast(
        &mut self,
        ui: &mut egui::Ui,
    ) {
        egui::ScrollArea::both()
            .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysVisible)
            .show(ui, |ui| {
                let mut focus_range: (Option<usize>,Option<usize>) = (None,None);
                if let Some((pos,phase)) = self.focus {
                    focus_range = tsmcc::show_span(&self.state.clone().unwrap(),pos,phase,SnapShotPhase::IrGenerator);
                }

                if let Some(snapshot) = self.state.clone() {
                    if snapshot.ast.count() > 0 {
                        self.render_node(ui,focus_range,snapshot.ast.get_root());
                    }
                }
            });
    }

    fn render_symtable(
        &mut self,
        ui: &mut egui::Ui,
    ) {
        egui::ScrollArea::vertical()
            .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysVisible)
            .show(ui, |ui| {
                if let Some(snapshot) = self.state.clone() {
                    ui.label(format!("{:#?}",snapshot.func_table));
                }
            });
    }


    fn render_ir(
        &mut self,
        ui: &mut egui::Ui,
    ) {
        egui::ScrollArea::vertical()
            .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysVisible)
            .show(ui, |ui| {
                let mut focus_range = (None,None);
                if let Some((pos,phase)) = self.focus {
                    focus_range = tsmcc::show_span(&self.state.clone().unwrap(),pos,phase,SnapShotPhase::CodeGenerator);
                }

                if let Some(snapshot) = self.state.clone() {

                    for (idx,line) in snapshot.iloc.as_iter().enumerate() {
                            
                        let t  = egui::RichText::new(format!("{:#?}",line))
                            .font(self.font_id.clone());
                        let button = egui::Button::new(t)
                            .fill(
                                if let  (Some(start),Some(end))= focus_range {
                                    if idx >= start && idx < end {
                                        Color32::RED
                                    } else {
                                        Color32::YELLOW
                                    }
                                        
                                } else {
                                    Color32::YELLOW
                                });

                        if ui.add(button).clicked() {
                            self.focus = Some((idx,SnapShotPhase::IrGenerator));
                        }
                            
                    }

                }
            });
    }

    fn render_asm(
        &mut self,
        ui: &mut egui::Ui,
    ) {
        egui::ScrollArea::vertical()
            .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysVisible)
            .show(ui, |ui| {
                let mut focus_range = (None,None);
                if let Some((pos,phase)) = self.focus {
                    focus_range = tsmcc::show_span(&self.state.clone().unwrap(),pos,phase,SnapShotPhase::Assembler);
                }

                if let Some(snapshot) = self.state.clone() {

                    for (idx,line) in snapshot.assembly.iter().enumerate() {
                            
                        let t  = egui::RichText::new(line.to_string())
                            .font(self.font_id.clone());

                        let button = egui::Button::new(t)
                            .fill(
                                if let  (Some(start),Some(end))= focus_range {
                                    if idx >= start && idx < end {
                                        Color32::RED
                                    } else {
                                        Color32::YELLOW
                                    }
                                        
                                } else {
                                    Color32::YELLOW
                                });

                        if ui.add(button).clicked() {
                            self.focus = Some((idx,SnapShotPhase::CodeGenerator));
                        }
                            
                    }

                }

            });
    }

    fn render_binary(
        &mut self,
        ui: &mut egui::Ui,
    ) {
        egui::ScrollArea::vertical()
            .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysVisible)
            .show(ui, |ui| {
                let mut focus_range = (None,None);
                if let Some((pos,phase)) = self.focus {
                    focus_range = tsmcc::show_span(&self.state.clone().unwrap(),pos,phase,SnapShotPhase::Finished);
                }

                if let Some(snapshot) = self.state.clone() {

                    for (idx,line) in snapshot.machine_code.as_iter().enumerate() {

                        let t  = egui::RichText::new(line.to_string())
                            .font(self.font_id.clone());

                        let button = egui::Button::new(t)
                            .fill(
                                if let  (Some(start),Some(end))= focus_range {
                                    if idx >= start && idx < end {
                                        Color32::RED
                                    } else {
                                        Color32::YELLOW
                                    }
                                        
                                } else {
                                    Color32::YELLOW
                                });

                        if ui.add(button).clicked() {
                            self.focus = Some((idx,SnapShotPhase::Assembler));
                        }   
                    }
                }   
            });
    }

    
    fn render_node(
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
            .unwrap_or_else(|| egui::collapsing_header::CollapsingState::load_with_default_open(ui.ctx(), id, false));

        let header_res = collapse_state.show_header(ui, |ui| {
            ui.horizontal(|ui| {            

                let t  = egui::RichText::new(format!("{:?}",cur_node.clone().get_kind()))
                    .font(self.font_id.clone());


                
                let button = egui::Button::new(t)
                    .fill(
                        if let  (Some(start),Some(end))= focus {
                            if node_id.to_usize() >= start && node_id.to_usize() < end {
                                Color32::RED
                            } else {
                                Color32::YELLOW
                            }
                                        
                        } else {
                            Color32::YELLOW
                        });
                if ui.add(button).clicked() {
                    self.focus = Some((node_id.to_usize(),SnapShotPhase::Parser));
                }
                
            });
        });
        header_res.body(|ui| {
            ui.indent(node_id.to_usize(), |ui| {
                if let Some(lchild) = cur_node.get_lchild() {
                    self.render_node(ui,focus,lchild);
                }
                if let Some(rchild) = cur_node.get_rchild() {
                    self.render_node(ui,focus,rchild);
                }
            });
        });


    }


    fn render_manual(
        &mut self,
        ui: &mut egui::Ui,
    ) {
        egui::ScrollArea::both()
            .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysVisible)
            .show(ui, |ui| {
                ui.add(egui::Image::new(egui::include_image!("../assets/output-01.png")));
                ui.add(egui::Image::new(egui::include_image!("../assets/output-02.png")));
                ui.add(egui::Image::new(egui::include_image!("../assets/output-03.png")));
                ui.add(egui::Image::new(egui::include_image!("../assets/output-04.png")));
                ui.add(egui::Image::new(egui::include_image!("../assets/output-05.png")));
                ui.add(egui::Image::new(egui::include_image!("../assets/output-06.png")));
                ui.add(egui::Image::new(egui::include_image!("../assets/output-07.png")));
                ui.add(egui::Image::new(egui::include_image!("../assets/output-08.png")));
                ui.add(egui::Image::new(egui::include_image!("../assets/output-09.png")));
                ui.add(egui::Image::new(egui::include_image!("../assets/output-10.png")));
                ui.add(egui::Image::new(egui::include_image!("../assets/output-11.png")));
                ui.add(egui::Image::new(egui::include_image!("../assets/output-12.png")));
                ui.add(egui::Image::new(egui::include_image!("../assets/output-13.png")));
                ui.add(egui::Image::new(egui::include_image!("../assets/output-14.png")));
                ui.add(egui::Image::new(egui::include_image!("../assets/output-15.png")));
                
            });

    }
    fn render_config(
        &mut self,
        ui: &mut egui::Ui,
    ) {
        let mut font_size: f32 = self.font_id.size;
        ui.add(egui::Slider::new(&mut font_size, 10.0..=100.0).text("폰트크기"));
        self.font_id = FontId::new(font_size, FontFamily::Monospace);
    }


}
