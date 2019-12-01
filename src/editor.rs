use crate::parameter::MyParameters;
//use crate::ui::MyUi;

use std::os::raw::c_void;
use std::path::Path;
use std::sync::Arc;

use derive_builder::Builder;

use vst::editor::Editor;

//use conrod::glium;

use crate::ui;
use ui::MyUi;

use ezui;
use ezui::glium;
use ezui::mouse::*;
use ezui::widget::*;
use ezui::winit;

use glium::backend::glutin_backend;
use glium::glutin;
use glium::DisplayBuild;
use glium::Surface;

#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct MyEditor {
    #[builder(default = "(400, 400)")]
    size: (i32, i32),

    params: Arc<MyParameters>,

    #[builder(default = "None", setter(skip))]
    ui: Option<MyUi>,
}

impl Editor for MyEditor {
    fn size(&self) -> (i32, i32) {
        self.size
    }

    fn position(&self) -> (i32, i32) {
        (0, 0)
    }

    fn idle(&mut self) {
        if let Some(ui) = &mut self.ui {
            let params = Arc::clone(&self.params);
            ui.update(params);
        }
    }

    fn close(&mut self) {
        self.ui = None;
    }

    fn open(&mut self, parent: *mut c_void) -> bool {
        let wb = winit::WindowBuilder::new()
            .with_dimensions(self.size.0 as u32, self.size.1 as u32)
            .with_decorations(false)
            .with_parent(parent);

        match glutin::WindowBuilder::from_winit_builder(wb).build_glium() {
            Ok(display) => {
                self.ui = Some(MyUi::new(display));
            }
            Err(_) => (),
        }

        self.is_open()
    }

    fn is_open(&mut self) -> bool {
        self.ui.is_some()
    }
}
