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

use glium::glutin;
use glium::glutin::GlContext;
use glium::Surface;

use winit::dpi::LogicalSize;

use winapi::shared::windef::HWND;
use winapi::shared::windef::HWND__;

use winit::os::windows::WindowBuilderExt;

#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct MyEditor {
    #[builder(default = "(400, 400)")]
    size: (i32, i32),

    params: Arc<MyParameters>,

    #[builder(default = "None", setter(skip))]
    ui: Option<Box<MyUi>>,
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
        simple_logging::log_to_file("M:/VST2/x64/Kazzix/log.txt", log::LevelFilter::Trace);
        use log::info;
        info!("open start");

        info!("parent: {}", parent as u32);
        let parent = parent as HWND;

        let events_loop = glutin::EventsLoop::new();
        let window_builder = glutin::WindowBuilder::new()
            .with_dimensions(LogicalSize::new(self.size.0 as f64, self.size.1 as f64))
            .with_decorations(false);
        //.with_parent_window(parent);

        let context_builder = glutin::ContextBuilder::new();

        info!("will build gl_window");
        let gl_window =
            glutin::GlWindow::new(window_builder, context_builder, &events_loop).unwrap();
        info!("built gl_window");

        unsafe { gl_window.make_current() }.unwrap();

        if let Ok(display) = glium::Display::from_gl_window(gl_window) {
            self.ui = Some(Box::new(MyUi::new(display, events_loop)));
        }

        self.is_open()
    }

    fn is_open(&mut self) -> bool {
        self.ui.is_some()
    }
}
