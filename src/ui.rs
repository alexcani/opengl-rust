use egui::Context;

pub struct Ui {
    pub quit: bool,
    pub camera_speed: f32,
    pub clear_color: [f32; 3],
    pub camera_sensitivity: f32,
}

impl Ui {
    pub fn new() -> Self {
        Ui { quit: false, camera_speed: 5.0, clear_color: [0.6, 0.4, 0.8], camera_sensitivity: 0.2 }
    }

    pub fn run(&mut self, ctx: &Context) {
        egui::Window::new("My Window").collapsible(false).show(ctx, |ui| {
            ui.add(egui::Slider::new(&mut self.camera_speed, 1.0..=20.0).text("Camera speed"));
            ui.add(egui::Slider::new(&mut self.camera_sensitivity, 0.1..=1.0).text("Camera sensitivity"));
            ui.color_edit_button_rgb(self.clear_color.as_mut().try_into().unwrap())
         });
    }
}

impl Default for Ui {
    fn default() -> Self {
        Self::new()
    }
}
