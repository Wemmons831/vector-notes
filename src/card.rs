use egui::{epaint::image, Align, Button, Color32, Image, ImageSource, Label, Layout, RichText, Ui};
#[derive(Clone, Default)]
pub struct Card{
    pub word: String,
    pub definition: String,
    pub showing: bool,
    is_image: bool,
    image_url: String,
}
impl Card {
    pub fn new( _word:String, _definition:String) -> Self{
        
        let _is_image:bool;
        let _image_url: String;
        if _definition[0..5] == "LATEX".to_string(){
             _is_image = true;
             _image_url =  ("https://latex.codecogs.com/png.image?".to_string() + &_definition[5..]);
        } 
        else{
             _is_image = false;
             _image_url = "".to_string();
        }
        Card { word: _word, definition: _definition, showing: (true), is_image: _is_image, image_url: _image_url }
    }
    pub fn render(&mut self,ui: &mut Ui, index: &mut usize){
        let text: String;
        if self.showing { text = self.word.to_owned();}
        else {text = self.definition.to_owned()}
        
        
            //ui.set_max_width(800f32);
            ui.with_layout(Layout::left_to_right(Align::Min),|ui: &mut Ui|{
                
            
            ui.add_space(ui.ctx().screen_rect().width()/2.0);
            if ui.add(Button::new("<".to_string())).clicked() && *index > 0usize{
                *index -= 1;
            }
            if !self.is_image || self.showing {
            if ui.add(Label::new(RichText::new(text).background_color(Color32::GOLD))).clicked(){
                self.showing = !self.showing;
            }}
            else {
                if ui.add(Image::from_uri(self.image_url.to_owned())).clicked(){
                    self.showing = !self.showing;
                }
            }
            if ui.add(Button::new(">".to_string())).clicked(){
                *index += 1;
            }
            });
            


    
    
    }
}


