

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
use crate::card;
use eframe::egui;
use egui::Button;
use egui::TextBuffer;
use futures::executor::block_on;
use wasm_bindgen_futures::spawn_local;
use std::sync::{Mutex, Arc};

use lazy_static;
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    word: String,
    definition: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    total: i32,
    known: i32,
    mastered: i32,
    #[serde(skip)]
    words: Vec<card::Card>,
    #[serde(skip)]
    index: usize,
    latex_example: String,
}
struct SharedData {
    value: String,
}
lazy_static::lazy_static! {
    static ref SHARED_DATA: Mutex<SharedData> = Mutex::new(SharedData { value: String::from("") });
}
impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            word: "".to_owned(),
            definition: "".to_owned(),
            total: 100,
            known: 0,
            mastered: 0,
            words: Vec::new(),
            index: 0,
            latex_example: "".to_string(),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
       

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        
        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(& mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            ui.horizontal(|ui| {
                ui.heading("Vector Notes");
                egui::widgets::global_dark_light_mode_buttons(ui);
            })
            
        });

        egui::CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("eframe templat");

            ui.horizontal(|ui: &mut egui::Ui| {
                ui.vertical( |ui:&mut egui::Ui|{
                    ui.label("Word: ");
                    ui.text_edit_singleline(&mut self.word);
                    ui.label("Definition: ");
                    ui.text_edit_singleline(&mut self.definition);
                });
                ui.vertical_centered( |ui: &mut egui::Ui|{
                    if ui.add(egui::Button::new("Add")).clicked(){
                    self.words.push(card::Card::new(self.word.to_owned(), self.definition.to_owned()));
                    self.word = String::from("");
                    self.definition = String::from("");
                    self.total += 1;
            

                    }
                    
                    if ui.add(egui::Button::new("Clea")).clicked(){
                        self.words = Vec::new();
                        self.total = 0;
                    }
                    if ui.add(egui::Button::new("file")).clicked(){
                        //let s = rfd::FileDialog::new().pick_file().unwrap();
                        
                        #[cfg(not(target_arch = "wasm32"))]
                        let v = block_on(rfd::AsyncFileDialog::new().pick_file()).unwrap();
                        #[cfg(not(target_arch = "wasm32"))]{
                        let s = match String::from_utf8(block_on(v.read())) {
                            Ok(e) =>e,
                            Err(e) => panic!("failed: {}", e)
                        };
                        let mut data = SHARED_DATA.lock().unwrap();
                        data.value = s;
                        }
                        
                        
                        #[cfg(target_arch = "wasm32")]{
                        spawn_local( async {
                            let v = rfd::AsyncFileDialog::new().pick_file();
                            let s = match String::from_utf8(v.await.unwrap().read().await) {
                                Ok(e) =>e,
                                Err(e) => panic!("failed: {}", e)
                            };
                            let mut data = SHARED_DATA.lock().unwrap();
                            data.value = s;
                        });
                        }
                        
                    }
                    });
                    
                let mut data= SHARED_DATA.lock().unwrap();
                if data.value != ""{
                    data.value.lines().for_each(|line| {
                            if line != "".as_str() {
                                let s:Vec<&str> = line.split(",").collect();
                                self.words.push(card::Card::new(s.get(0).unwrap().to_string(),s.get(1).unwrap().to_string()));
                            }
                        });
                } 
            });

            

            ui.separator();
            let  i = self.words.get_mut(self.index);
           
            i.unwrap_or(&mut card::Card::new( String::from("ERROR"), String::from("ERROR"))).render(ui,&mut self.index);
            
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
        
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
