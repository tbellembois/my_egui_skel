use crate::ui::{app::App, pages::product, state::Page};
use egui::{Frame, RichText};
use rust_i18n::t;

pub fn update(app: &mut App, ctx: &egui::Context, frame: &mut eframe::Frame) {
    //
    // Render top panel with info/error message, menu, theme switcher and locale switcher.
    //
    egui::TopBottomPanel::top("info_error_panel")
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

            // Switch locale and theme.
            ui.horizontal(|ui| {
                // Switch locale.
                let locales = rust_i18n::available_locales!();
                for (i, locale) in locales.iter().enumerate() {
                    if ui
                        .selectable_value(&mut app.state.active_locale, i.to_string(), *locale)
                        .changed()
                    {
                        rust_i18n::set_locale(locale);
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Title(
                            t!("My egui App").to_string(),
                        ));
                    }
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
            });

            // Render menu.
            egui::menu::bar(ui, |ui| {
                ui.menu_button(t!("bookmarks"), |ui| {
                    if ui.button("Save").clicked() {
                        //functionality
                    }
                    if ui.button("Quit").clicked() {
                        std::process::exit(0);
                    }
                });

                ui.menu_button("Edit", |ui| {
                    if ui.button("Cut").clicked() {
                        //functionality
                    }
                    if ui.button("Copy").clicked() {
                        //functionality
                    }
                    if ui.button("Paste").clicked() {
                        //funtionality
                    }
                })
            });
        });

    //
    // Render active page.
    //
    egui::CentralPanel::default()
        .frame(Frame {
            inner_margin: app.state.active_theme.margin_style().into(),
            fill: app.state.active_theme.bg_primary_color_visuals(),
            stroke: egui::Stroke::new(1.0, app.state.active_theme.bg_secondary_color_visuals()),
            ..Default::default()
        })
        .show(ctx, |ui| match app.state.active_page {
            Page::ProductList => product::list::update(app, ctx, frame, ui),
        });
}
