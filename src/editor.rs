use crate::parameter::MyParameters;
//use crate::ui::MyUi;

use std::os::raw::c_void;
use std::path::Path;
use std::sync::Arc;

use derive_builder::Builder;

use vst::editor::Editor;

//use conrod::glium;

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
pub struct MyEditor<'a> {
    #[builder(default = "(400, 400)")]
    size: (i32, i32),

    params: Arc<MyParameters>,

    #[builder(default = "None", setter(skip))]
    ui: Option<ezui::Ui>,

    texture_knob_attack_base: ezui::standard::UiTexture<'a>,
    texture_knob_attack_light: ezui::standard::UiTexture<'a>,
    button_knob_attack: ezui::standard::UiButton,
}

impl<'a> Editor for MyEditor<'a> {
    fn size(&self) -> (i32, i32) {
        self.size
    }

    fn position(&self) -> (i32, i32) {
        (0, 0)
    }

    fn idle(&mut self) {
        /*
        let params = Arc::clone(&self.params);
        let texture_knob_attack_base = &mut self.texture_knob_attack_base;
        let texture_knob_attack_light = &mut self.texture_knob_attack_light;
        let button_knob_attack = &mut self.button_knob_attack;
        if let Some(ui) = &mut self.ui {
            ui.update(|target, events, mouse, system| {
                ///// UPDATE /////
                button_knob_attack.update(mouse);
                button_knob_attack
                    .state()
                    .iter()
                    .for_each(|(button, state)| match state {
                        ButtonState::Pressed(_, _) => {
                            let scale = match button {
                                MouseButton::Left => 0.2,
                                _ => 1.0,
                            };
                            let attack = params.get_attack();
                            let attack = attack + mouse.delta_position().1 * scale;
                            params.set_attack(attack);
                            texture_knob_attack_light.rotation = attack * 270.0 - 180.0;
                        }
                        _ => (),
                    });

                ///// DRAW /////
                target.clear_color(0.2, 0.2, 0.2, 0.2);

                texture_knob_attack_base.draw(target, system);
                texture_knob_attack_light.draw(target, system);
            });
        }
        */
    }

    fn close(&mut self) {
        self.ui = None;
    }

    fn open(&mut self, parent: *mut c_void) -> bool {
        let wb = winit::WindowBuilder::new()
            .with_dimensions(self.size.0 as u32, self.size.1 as u32)
            .with_decorations(false)
            .with_parent(parent);

        match glutin::WindowBuilder::from_winit_builder(wb)
            .with_multisampling(4)
            .build_glium()
        {
            Ok(display) => {
                self.ui = Some(ezui::Ui::new(display));
            }
            Err(_) => (),
        }

        //self.is_open()
        false
    }

    fn is_open(&mut self) -> bool {
        self.ui.is_some()
    }
}
