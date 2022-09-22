use eframe::{
    egui::{Context, ColorImage},
    glow::{HasContext, self},
    egui::widgets::plot::PlotImage,
    egui::{TextureHandle, TextureFilter,CentralPanel, plot::PlotPoint},
    egui::mutex::Mutex,
};
use epaint::{TextureManager, Vec2};
use egui_extras::RetainedImage;
use flume::{Sender, Receiver};
use std::sync::Arc;
use crate::services::MirrorService;

pub struct MyApp {
    texture: Option<TextureHandle>,
    service: MirrorService,
    service_is_run: bool,
    img_recver: Receiver<ColorImage>
}

impl  MyApp {
    pub fn new() -> Self {
        let (img_sender, img_recver) = flume::unbounded();
        Self {
            texture: None,
            service: MirrorService::new(None, img_sender),
            service_is_run: false,
            img_recver,
        }
    }

    pub fn service_start(&mut self, mut ctx: Context) {
        self.service_is_run = true;
        self.service.run()
    }
   
}


impl eframe::App for MyApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        if !self.service_is_run {
            self.service_start(ctx.clone()) ;   
        }
        CentralPanel::default().show(ctx, |ui| {
            match self.img_recver.try_recv() {
                Ok(img) => {self.texture = Some(ui.ctx().load_texture(
                    "remote desktop",
                    img,
                    TextureFilter::Linear,
                ));     
                }
                Err(_) => ()
            }
            if let Some(texture) = self.texture.as_ref() {
                ui.image(texture, ui.available_size());
            }
            
        });
        
        
        

    }

}