use eframe::{egui, App, Frame, NativeOptions};

struct MenuApp {
    menu_items: Vec<String>,
    selected_item: Option<String>,
}

impl Default for MenuApp {
    fn default() -> Self {
        Self {
            menu_items: vec![
                "Start".into(),
                "Settings".into(),
                "Help".into(),
                "Quit".into(),
            ],
            selected_item: None,
        }
    }
}

impl App for MenuApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Welcome To The Main Menu");

            for item in &self.menu_items {
                if ui.button(item).clicked() {
                    self.selected_item = Some(item.clone());
                }
            }

            if let Some(selected) = &self.selected_item {
                ui.separator();
                ui.label(format!("You selected: {}", selected));
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    let native_options = NativeOptions::default();
    eframe::run_native(
        "New app menu",
        native_options,
        Box::new(|_cc| Ok(Box::new(MenuApp::default()))),
    )
}
