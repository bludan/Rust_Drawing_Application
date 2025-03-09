use eframe::egui;

struct MyApp {
    pixels: Vec<(f32, f32)>, // Stores the positions of clicked pixels
    size: f32,
    shape: ShapeType,
    prev_mouse_pos: Option<egui::Pos2>,
    drawing_enabled: bool,
}
#[derive(PartialEq)]
enum ShapeType {
    Circle,
    Rectangle,
}
impl Default for MyApp {
    fn default() -> Self {
        Self {
            pixels: Vec::new(),
            size: 1.0,
            shape: ShapeType::Circle,
            prev_mouse_pos: None,
            drawing_enabled: false,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
       egui::SidePanel::left("left_panel").max_width(100.0).show(ctx, |ui|{
        ui.heading("Control Panel");
           if ui.add_sized([50.0, 40.0],egui::Button::new("+")).clicked() {
               self.size = self.size + 1.0;
           }
           if ui.add_sized([50.0, 40.0],egui::Button::new("-")).clicked() {
               self.size = self.size - 1.0;
           }
           if ui.add_sized([80.0, 40.0],egui::Button::new("rectangle")).clicked() {
               self.shape = ShapeType::Rectangle;
           }
           if ui.add_sized([80.0, 40.0],egui::Button::new("circle")).clicked() {
               self.shape = ShapeType::Circle;
           }
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            self.drawing_enabled = true;
            if ui.input(|input| input.pointer.primary_down() && self.drawing_enabled == true) {
                if let Some(pos) = ui.input(|input| input.pointer.interact_pos()) {
                    if let Some(prev_pos) = self.prev_mouse_pos {
                        let distance = prev_pos.distance(pos);
                        let steps = (distance / self.size).ceil() as usize;
                        for i in 0..=steps {
                            let t = i as f32 / steps as f32;
                            let x = prev_pos.x + t * (pos.x - prev_pos.x);
                            let y = prev_pos.y + t * (pos.y - prev_pos.y);
                            self.pixels.push((x, y));
                        }
                    } else {
                        self.pixels.push((pos.x, pos.y));
                    }
                    self.prev_mouse_pos = Some(pos);
                } else {
                    self.prev_mouse_pos = None;
                }
            } else {
                self.prev_mouse_pos = None;
            }
            let painter = ui.painter();
            for &(x, y) in &self.pixels {
                if self.shape == ShapeType::Circle {
                    painter.circle_filled(egui::Pos2::new(x, y), self.size, egui::Color32::RED);
                } else if self.shape == ShapeType::Rectangle {
                    painter.rect_filled(
                        egui::Rect::from_center_size(
                            egui::Pos2::new(x, y),
                            egui::Vec2::new(self.size * 2.0, self.size * 2.0),
                        ),
                        0.0,
                        egui::Color32::RED,
                    );
                }
            }
        });
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Simple egui Pixel Drawer",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    );
}
