use crate::services::MirrorService;
use eframe::{
    egui::mutex::Mutex,
    egui::containers::Frame,
    egui::style::Margin,
    egui::widgets::plot::PlotImage,
    egui::{plot::PlotPoint, CentralPanel, TextureFilter, TextureHandle},
    egui::{ColorImage, Context},
    glow::{self, HasContext},
};
use egui_extras::RetainedImage;
use epaint::{TextureManager, Vec2};
use flume::{Receiver, Sender};
use std::sync::Arc;

pub struct MyApp {
    texture: Option<TextureHandle>,
    service: MirrorService,
    service_is_run: bool,
    img_recver: Receiver<Box<ColorImage>>,
}

impl MyApp {
    pub fn new() -> Self {
        let (img_sender, img_recver) = flume::unbounded();
        Self {
            texture: None,
            service: MirrorService::new(img_sender),
            service_is_run: false,
            img_recver,
        }
    }

    pub fn service_start(&mut self, mut ctx: Context) {
        self.service_is_run = true;
        self.service.run(ctx)
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        if !self.service_is_run {
            self.service_start(ctx.clone());
        }
        let mut new_frame = Frame::none().inner_margin(Margin{left:0.0, right:0.0, top:0.0, bottom:0.0});
        CentralPanel::default().frame(new_frame)
        .show(ctx, |ui| {
            match self.img_recver.try_recv() {
                Ok(mut boxed_img) => {
                    //let img = boxed_img.as_mut();
                    self.texture = Some(ui.ctx().load_texture(
                        "remote desktop",
                        *boxed_img,//img.to_owned(),
                        TextureFilter::Linear,
                    ));
                }
                Err(_) => (),
            }
            if let Some(texture) = self.texture.as_ref() {
                ui.image(texture, ui.available_size());
                
            }
        });
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {}

    fn on_close_event(&mut self) -> bool {
        true
    }

    fn on_exit(&mut self, _gl: Option<&glow::Context>) {}

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(30)
    }

    fn max_size_points(&self) -> eframe::egui::Vec2 {
        eframe::egui::Vec2::INFINITY
    }

    fn clear_color(&self, _visuals: &eframe::egui::Visuals) -> eframe::egui::Rgba {
        // NOTE: a bright gray makes the shadows of the windows look weird.
        // We use a bit of transparency so that if the user switches on the
        // `transparent()` option they get immediate results.
        eframe::egui::Color32::from_rgba_unmultiplied(12, 12, 12, 180).into()

        // _visuals.window_fill() would also be a natural choice
    }

    fn persist_native_window(&self) -> bool {
        true
    }

    fn persist_egui_memory(&self) -> bool {
        true
    }

    fn warm_up_enabled(&self) -> bool {
        false
    }

    fn post_rendering(&mut self, _window_size_px: [u32; 2], _frame: &eframe::Frame) {}
}
