use egui::{RichText, Ui};
#[derive(Clone, Default)]
pub struct Card{
    pub word: String,
    pub definition: String,
    pub showing: bool,
}
impl Card {
    pub fn render(&mut self,ui: &mut Ui){
        let text: String;
        if self.showing { text = self.word.to_owned();}
        else {text = self.definition.to_owned()}
        ui.horizontal_top(|ui|{
            if ui.add(egui::Button::new(RichText::new(text).size(128.0))).clicked(){
                self.showing = !self.showing;
                
            }


        });
    
    }
}


