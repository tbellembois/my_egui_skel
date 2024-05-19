use crate::{
    error::apperror::AppError,
    ui::myapp::Myapp,
    worker::message::{ToWorker, ToWorkerMessage},
};
use egui::{Frame, RichText};
use log::error;

pub fn update(app: &mut Myapp, ctx: &egui::Context, frame: &mut eframe::Frame) {
    egui::TopBottomPanel::top("my_panel")
        .min_height(40.)
        .max_height(40.)
        .show_separator_line(true)
        .frame(Frame {
            inner_margin: app.state.active_theme.margin_style().into(),
            fill: app.state.active_theme.bg_secondary_color_visuals(),
            stroke: egui::Stroke::new(1.0, app.state.active_theme.bg_secondary_color_visuals()),
            ..Default::default()
        })
        .show(ctx, |ui| {
            // Display possible error.
            if let Some(error) = &app.current_error {
                ui.label(
                    RichText::new(format!(" {}", error))
                        .color(app.state.active_theme.fg_error_text_color_visuals()),
                );
            }

            // Display possible message.
            if let Some(info) = &app.current_info {
                ui.label(
                    RichText::new(format!(" {}", info))
                        .color(app.state.active_theme.fg_success_text_color_visuals()),
                );
            }

            // Switch theme.
            egui::ComboBox::from_id_source("settings_theme_combo_box")
                .width(200.0)
                .selected_text(app.state.active_theme.name())
                .show_ui(ui, |ui_combobox| {
                    for theme in app.themes.iter() {
                        let res: egui::Response = ui_combobox.selectable_value(
                            &mut app.state.active_theme,
                            theme.clone(),
                            theme.name(),
                        );
                        if res.changed() {
                            ui_combobox
                                .ctx()
                                .set_style(app.state.active_theme.custom_style());
                        }
                    }
                });

            // Ferris logo.
            ui.add_sized(
                [200., 70.],
                egui::Image::new(egui::include_image!("../../media/ferris.svg")),
            );

            // User name input.
            ui.horizontal(|ui| {
                ui.label("enter your name");
                ui.add_sized(
                    [400., 30.],
                    egui::TextEdit::singleline(&mut app.user_name_input)
                        .hint_text("firstname and lastname"),
                );
            });

            // Say hello button.
            let button = egui::Button::new("say hello");
            if ui.add_sized([150., 30.], button).clicked() {
                app.user_name = Some(app.user_name_input.clone());
            }

            // Hello.
            if let Some(user_name) = app.user_name.clone() {
                ui.label(format!("hello {}!", user_name));
            }

            // Ping button.
            let button = egui::Button::new("ping");
            if ui.add_sized([150., 30.], button).clicked() {
                let mayerr_send = app.sender.as_ref().unwrap().send(ToWorker {
                    message: ToWorkerMessage::Ping,
                });

                if let Err(e) = mayerr_send {
                    error!("error sending ping: {e}");
                    app.current_error = Some(AppError::ChannelSendError(e.to_string()));
                }
            }

            // Trigger error button.
            let button = egui::Button::new("trigger error");
            if ui.add_sized([150., 30.], button).clicked() {
                app.current_error = Some(AppError::InternalError(
                    "something wrong happened".to_string(),
                ))
            }
        });
}
