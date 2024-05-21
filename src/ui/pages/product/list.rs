use crate::api::hackernews;
use crate::ui::app::App;
use egui::Ui;
use rust_i18n::t;

pub fn update(app: &mut App, ctx: &egui::Context, frame: &mut eframe::Frame, ui: &mut Ui) {
    ui.vertical(|ui| {
        // Ferris logo.
        ui.add_sized(
            [300., 70.],
            egui::Image::new(egui::include_image!("../../media/ferris.svg")),
        );

        // Get hacker news button.
        let button = egui::Button::new(t!("button_hacker_news"));
        if ui.add_sized([150., 30.], button).clicked() {
            app.current_info = Some(t!("fetch_hacker_news").to_string());
            app.promise_hacker_news = Some(hackernews::retrieve_feeds(ctx));
        };

        // List news.
        if let Some(hacker_news) = &app.hacker_news {
            ui.label(hacker_news);
        };
    });
}
