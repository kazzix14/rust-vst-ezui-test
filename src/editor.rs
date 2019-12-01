use crate::parameter::MyParameters;
//use crate::ui::MyUi;

use std::os::raw::c_void;
use std::path::Path;
use std::sync::Arc;

use derive_builder::Builder;

use vst::editor::Editor;

//use conrod::glium;

use crate::ui;

use ezui;
use ezui::glium;
use ezui::mouse::*;
use ezui::widget::*;
use ezui::winit;

use glium::backend::glutin_backend;
use glium::glutin;
use glium::DisplayBuild;
use glium::Surface;

pub const FONT_RAW: &'static [u8] = include_bytes!("../resource/font/OpenSans-Regular.ttf");
pub const KNOB_BASE_WHITE_RAW: &'static [u8] = include_bytes!("../resource/image/white.png");
pub const KNOB_LIGHT_RAW: &'static [u8] = include_bytes!("../resource/image/light.png");

#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct MyEditor {
    #[builder(default = "(400, 400)")]
    size: (i32, i32),

    params: Arc<MyParameters>,

    #[builder(default = "None", setter(skip))]
    ui: Option<ezui::Ui>,

    #[builder(default = "None", setter(skip))]
    texture_knob_attack_base: Option<ezui::standard::UiTexture>,
    #[builder(default = "None", setter(skip))]
    texture_knob_attack_light: Option<ezui::standard::UiTexture>,
    #[builder(default = "None", setter(skip))]
    button_knob_attack: Option<ezui::standard::UiButton>,
}

impl Editor for MyEditor {
    fn size(&self) -> (i32, i32) {
        self.size
    }

    fn position(&self) -> (i32, i32) {
        (0, 0)
    }

    fn idle(&mut self) {
        let params = Arc::clone(&self.params);
        let mut texture_knob_attack_base = self.texture_knob_attack_base.as_mut();
        let mut texture_knob_attack_light = self.texture_knob_attack_light.as_mut();
        let mut button_knob_attack = self.button_knob_attack.as_mut();

        macro_rules! unwrap_or_return {
            ($name:ident) => {
                if $name.is_none() {
                    return;
                }
                let $name = $name.as_mut().unwrap();
            };
        }

        unwrap_or_return!(texture_knob_attack_base);
        unwrap_or_return!(texture_knob_attack_light);
        unwrap_or_return!(button_knob_attack);

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
                                MouseButton::Left => 1.0,
                                _ => 5.0,
                            };
                            let attack = params.get_attack();
                            let attack = attack + mouse.delta_position().1 * scale;
                            params.set_attack(attack);
                            texture_knob_attack_light.rotation = attack * 270.0 - 180.0;
                        }
                        _ => (),
                    });
                ///// DRAW /////
                target.clear_color(0.2, 0.2, 0.2, 1.0);

                texture_knob_attack_base.draw(target, system);
                texture_knob_attack_light.draw(target, system);
            });
        }
    }

    fn close(&mut self) {
        self.ui = None;
        self.texture_knob_attack_base = None;
        self.texture_knob_attack_light = None;
        self.button_knob_attack = None;
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
                /*
                let position_knob_attack = (0.1, 0.1);
                let size_knob_attack = (0.3, 0.3);

                let texture_knob_attack_base_raw = ezui::SimpleTexture::from(
                    &KNOB_BASE_WHITE_RAW,
                    ezui::ImageFormat::PNG,
                    &display,
                )
                .unwrap();

                let texture_knob_attack_light_raw =
                    ezui::SimpleTexture::from(&KNOB_LIGHT_RAW, ezui::ImageFormat::PNG, &display)
                        .unwrap();

                let texture_knob_attack_base = ezui::standard::UiTextureBuilder::default()
                    .position(position_knob_attack)
                    .size(size_knob_attack)
                    .texture(Box::new(texture_knob_attack_base_raw))
                    .rotation(0.0)
                    .build()
                    .unwrap();

                let texture_knob_attack_light = ezui::standard::UiTextureBuilder::default()
                    .position(position_knob_attack)
                    .size(size_knob_attack)
                    .texture(Box::new(texture_knob_attack_light_raw))
                    .rotation(0.0)
                    .build()
                    .unwrap();

                let button_knob_attack = ezui::standard::UiButtonBuilder::default()
                    .position(position_knob_attack)
                    .size(size_knob_attack)
                    .build()
                    .unwrap();

                self.texture_knob_attack_base = Some(texture_knob_attack_base);
                self.texture_knob_attack_light = Some(texture_knob_attack_light);
                self.button_knob_attack = Some(button_knob_attack);

                self.ui = Some(ezui::Ui::new(display));
                */
            }
            Err(_) => (),
        }

        self.is_open()
    }

    fn is_open(&mut self) -> bool {
        self.ui.is_some()
    }
}
