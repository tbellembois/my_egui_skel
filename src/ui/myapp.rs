use super::state::{ApplicationState, Page};
use crate::{
    error::apperror::AppError,
    ui::pages::main,
    worker::{
        builder::Worker,
        message::{ToApp, ToAppMessage, ToWorker},
    },
};
use eframe::CreationContext;
use egui_aesthetix::{
    themes::{CarlDark, StandardDark, StandardLight},
    Aesthetix,
};
use std::{
    rc::Rc,
    sync::{
        mpsc::{self, Receiver, Sender},
        Once,
    },
    thread,
};

static START: Once = Once::new();

#[derive(Default)]
pub struct Myapp {
    // Application state.
    pub state: ApplicationState,

    // Channels for communication beetween
    // application (GUI) and worker.
    pub sender: Option<Sender<ToWorker>>,
    receiver: Option<Receiver<ToApp>>,

    // Holds the supported themes that the user can switch between.
    pub themes: Vec<Rc<dyn Aesthetix>>,

    // Current error if one.
    pub current_error: Option<AppError>,
    // Current info if one.
    pub current_info: Option<String>,

    // User name input.
    pub user_name_input: String,
    // User name.
    pub user_name: Option<String>,
}

impl Myapp {
    pub fn new(cc: &CreationContext) -> Self {
        // Create channels.
        let (app_tx, app_rx) = mpsc::channel();
        let (worker_tx, worker_rx) = mpsc::channel();

        dbg!("Spawning new worker.");

        // Spawn a thread with a new worker.
        let context = cc.egui_ctx.clone();
        thread::spawn(move || {
            Worker::new(worker_tx, app_rx, context).init();
        });

        dbg!("New worker spawned.");

        // Load custom fonts and styles.
        setup_custom_fonts(&cc.egui_ctx);

        // Load themes.
        let themes: Vec<Rc<dyn Aesthetix>> = vec![
            Rc::new(StandardDark),
            Rc::new(StandardLight),
            Rc::new(CarlDark),
        ];
        let active_theme: Rc<dyn Aesthetix> = match themes.first() {
            Some(theme) => theme.clone(),
            None => panic!("The first theme in the list of available themes could not be loaded."),
        };

        // Create application state.
        let state = ApplicationState::new(active_theme);

        // Initialize the custom theme/styles for egui.
        cc.egui_ctx.set_style(state.active_theme.custom_style());

        // Create application.
        Myapp {
            sender: Some(app_tx),
            receiver: Some(worker_rx),
            themes,
            ..Default::default()
        }
    }
}

impl eframe::App for Myapp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Check for ToApp messages.
        if let Some(receiver) = &self.receiver {
            let maybe_message = match receiver.try_recv() {
                Ok(message) => Some(message),
                Err(e) => match e {
                    mpsc::TryRecvError::Empty => None,
                    mpsc::TryRecvError::Disconnected => {
                        self.current_error = Some(AppError::ChannelClosed);
                        None
                    }
                },
            };

            if let Some(message) = maybe_message {
                println!("received = {:?}", message);
                match message.message {
                    ToAppMessage::Pong => self.current_info = Some("pong".to_string()),
                    ToAppMessage::Error(e) => self.current_error = Some(e), //FIXME: handle fatal errors
                }
            }
        }

        // Do one time startup job.
        START.call_once(|| {});

        // Render active page.
        match self.state.active_page {
            Page::Main => main::ui::update(self, ctx, frame),
        }
    }
}

fn setup_custom_fonts(ctx: &egui::Context) {
    // Start with the default fonts (we will be adding to them rather than replacing them).
    let mut fonts = egui::FontDefinitions::default();

    // Install custom fonts.
    // .ttf and .otf files supported.
    fonts.font_data.insert(
        "Font-Awesome-6-Brands-Regular-400".to_owned(),
        egui::FontData::from_static(include_bytes!(
            "fonts/Font-Awesome-6-Brands-Regular-400.otf"
        )),
    );
    fonts.font_data.insert(
        "Font-Awesome-6-Free-Regular-400".to_owned(),
        egui::FontData::from_static(include_bytes!("fonts/Font-Awesome-6-Free-Regular-400.otf")),
    );
    fonts.font_data.insert(
        "Font-Awesome-6-Free-Solid-900".to_owned(),
        egui::FontData::from_static(include_bytes!("fonts/Font-Awesome-6-Free-Solid-900.otf")),
    );

    // Start at 1 not 0 to keep the default font.
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(1, "Font-Awesome-6-Brands-Regular-400".to_owned());
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(2, "Font-Awesome-6-Free-Regular-400".to_owned());
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(3, "Font-Awesome-6-Free-Solid-900".to_owned());

    // Tell egui to use these fonts:
    ctx.set_fonts(fonts);
}
