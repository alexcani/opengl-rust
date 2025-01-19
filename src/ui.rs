use egui::Context;

pub struct Ui {
    pub quit: bool,
    pub camera_speed: f32,
    pub clear_color: [f32; 3],
    pub camera_sensitivity: f32,
    pub light_color: [f32; 3],
    pub shininess: i32,
    pub fps: u32,
}

impl Ui {
    pub fn new() -> Self {
        Ui {
            quit: false,
            camera_speed: 5.0,
            clear_color: [0.0, 0.0, 0.0],
            camera_sensitivity: 0.4,
            light_color: [1.0, 1.0, 1.0],
            shininess: 32,
            fps: 0,
        }
    }

    pub fn run(&mut self, ctx: &Context) {
        egui::Window::new("Controls")
            .collapsible(false)
            .show(ctx, |ui| {
                ui.label(format!("FPS: {}", self.fps));
                ui.add(egui::Slider::new(&mut self.camera_speed, 1.0..=20.0).text("Camera speed"));
                ui.add(
                    egui::Slider::new(&mut self.camera_sensitivity, 0.1..=1.0)
                        .text("Camera sensitivity"),
                );
                ui.add(egui::Slider::new(&mut self.shininess, 2..=256).text("Specular shininess"));
                ui.horizontal(|ui| {
                    ui.label("Light color:");
                    ui.color_edit_button_rgb(self.light_color.as_mut().try_into().unwrap());
                });
                ui.horizontal(|ui| {
                    ui.label("Clear color:");
                    ui.color_edit_button_rgb(self.clear_color.as_mut().try_into().unwrap())
                });
            });
    }
}

impl Default for Ui {
    fn default() -> Self {
        Self::new()
    }
}
