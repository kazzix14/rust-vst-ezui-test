use crate::parameter::MyParameters;
use vst::plugin::PluginParameters;

use std::sync::Arc;

use ezui::glium::backend::glutin::glutin::EventsLoop;
use ezui::glium::backend::glutin::Display;
use ezui::glium::Surface;
use ezui::mouse::*;
use ezui::standard::*;
use ezui::widget::*;
use ezui::*;

pub const FONT_RAW: &'static [u8] = include_bytes!("../resource/font/OpenSans-Regular.ttf");
pub const KNOB_BASE_WHITE_RAW: &'static [u8] = include_bytes!("../resource/image/white.png");
pub const KNOB_LIGHT_RAW: &'static [u8] = include_bytes!("../resource/image/light.png");

pub struct MyUi {
    ui: Ui,
    //texture_raw_attack_knob_base: Box<SimpleTexture>,
    //texture_raw_attack_knob_light: Box<SimpleTexture>,
    texture_attack_knob_base: UiTexture,
    texture_attack_knob_light: UiTexture,
    button_attack_knob: UiButton,
}

impl MyUi {
    pub fn new(display: Display, events_loop: EventsLoop) -> Self {
        const KNOB_ATTACK_POSITION: (f32, f32) = (0.1, 0.1);
        const KNOB_ATTACK_SIZE: (f32, f32) = (0.3, 0.3);
        const KNOB_ATTACK_ROTATION: f32 = 0.0;

        let texture_raw_attack_knob_base =
            SimpleTexture::from(&KNOB_BASE_WHITE_RAW, ImageFormat::PNG, &display).unwrap();
        let texture_raw_attack_knob_light =
            SimpleTexture::from(&KNOB_LIGHT_RAW, ImageFormat::PNG, &display).unwrap();
        let texture_raw_attack_knob_base = Arc::new(texture_raw_attack_knob_base);
        let texture_raw_attack_knob_light = Arc::new(texture_raw_attack_knob_light);

        let ui = Ui::new(display, events_loop);

        let texture_attack_knob_base = UiTextureBuilder::default()
            .position(KNOB_ATTACK_POSITION)
            .size(KNOB_ATTACK_SIZE)
            .rotation(KNOB_ATTACK_ROTATION)
            .texture(texture_raw_attack_knob_base)
            .build()
            .unwrap();
        let texture_attack_knob_light = UiTextureBuilder::default()
            .position(KNOB_ATTACK_POSITION)
            .size(KNOB_ATTACK_SIZE)
            .rotation(KNOB_ATTACK_ROTATION)
            .texture(texture_raw_attack_knob_light)
            .build()
            .unwrap();
        let button_attack_knob = UiButtonBuilder::default()
            .position(KNOB_ATTACK_POSITION)
            .size(KNOB_ATTACK_SIZE)
            .build()
            .unwrap();

        Self {
            ui,
            //texture_raw_attack_knob_base:            texture_raw_attack_knob_base,
            //texture_raw_attack_knob_light:            texture_raw_attack_knob_light,
            texture_attack_knob_base,
            texture_attack_knob_light,
            button_attack_knob,
        }
    }

    pub fn update(&mut self, params: Arc<MyParameters>) {
        let texture_attack_knob_base = &mut self.texture_attack_knob_base;
        let texture_attack_knob_light = &mut self.texture_attack_knob_light;
        let button_attack_knob = &mut self.button_attack_knob;
        let ui = &mut self.ui;
        use crate::Index;

        ui.update(|target, events, mouse, system| {
            ///// UPDATE /////
            button_attack_knob.update(mouse);
            button_attack_knob
                .state()
                .iter()
                .for_each(|(button, state)| match state {
                    ButtonState::Pressed(_, _) => {
                        let scale = match button {
                            MouseButton::Left => 1.0,
                            _ => 5.0,
                        };
                        let attack = params.get_parameter(Index::Attack as i32);
                        let attack = attack + mouse.delta_position().1 * scale;
                        params.set_parameter(Index::Attack as i32, attack);
                        texture_attack_knob_light.rotation = attack * 270.0 - 180.0;
                    }
                    _ => (),
                });
            ///// DRAW /////
            target.clear_color(0.2, 0.2, 0.2, 1.0);

            texture_attack_knob_base.draw(target, system);
            texture_attack_knob_light.draw(target, system);
        });
    }
}
