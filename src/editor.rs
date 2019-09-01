use crate::parameter::MyParameters;
use crate::ui::MyUi;

use std::path::Path;
use std::sync::Arc;
use std::os::raw::c_void;

use derive_builder::Builder;

use vst::editor::Editor;

use conrod::glium;

use glium::DisplayBuild;


#[derive(Builder)]
pub struct MyEditor {
    #[builder(default = "(200, 200)")]
    size: (i32, i32),

    #[builder(default = "(0, 0)")]
    position: (i32, i32),

    #[builder(default = "false", setter(skip))]
    is_open: bool,

    params: Arc<MyParameters>,

    #[builder(default = "None", setter(skip))]
    ui: Option<MyUi>,
}

impl Editor for MyEditor {
    fn size(&self) -> (i32, i32) {
        self.size
    }

    fn position(&self) -> (i32, i32) {
        self.position
    }

    fn idle(&mut self) {
        if let Some(ui) = &mut self.ui {
            ui.draw(Arc::clone(&self.params));
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

        match conrod::glium::glutin::WindowBuilder::from_winit_builder(wb)
            .with_multisampling(4)
            .build_glium()
        {
            Ok(display) => self.ui = Some(MyUi::new(Path::new("."), display)),
            Err(_) => (),
        }

        self.is_open()
    }

    fn is_open(&mut self) -> bool {
        self.ui.is_some()
    }
}
