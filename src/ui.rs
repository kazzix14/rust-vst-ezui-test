use crate::parameter::MyParameters;

use std::path::Path;
use std::sync::Arc;

use conrod;
use conrod::glium;
use conrod::widget_ids;

widget_ids! {
    struct Ids{
        body,
        chick,
        attack_slider,
    }
}

pub struct MyUi {
    display: glium::Display,
    ui: conrod::Ui,
    image_map: conrod::image::Map<glium::texture::Texture2d>,
    ids: Ids,
    renderer: conrod::backend::glium::Renderer,
}


impl MyUi {
    pub fn new(my_folder: &Path, display: glium::Display) -> Self {
        let (width, height) = display
            .get_window()
            .and_then({ |window| window.get_inner_size() })
            .unwrap();

        //info!("size : {}x{}", width, height);

        //info!("framebuffer: {:?}", display.get_framebuffer_dimensions());

        let mut ui = conrod::UiBuilder::new([width as f64, height as f64]).build();

        ui.fonts
            .insert_from_file(Path::new("M:/VST2/x64/Kazzix/font.ttf"))
            .unwrap();

        let renderer = match conrod::backend::glium::Renderer::new(&display) {
            Ok(r) => r,
            Err(e) => {
                //error!("Error creating Renderer: {:?}", e);
                //return Err(AppError::LoadRendererFail)
                panic!();
            }
        };

        let image_map = conrod::image::Map::new();
        let ids = Ids::new(ui.widget_id_generator());

        MyUi {
            display: display,
            ui: ui,
            image_map: image_map,
            renderer: renderer,
            ids: ids,
        }
    }

    pub fn draw(&mut self, params: Arc<MyParameters>) {
        for event in self.display.poll_events() {
            // Use the `winit` backend feature to convert the winit event to a conrod one.
            if let Some(event) = conrod::backend::winit::convert(event.clone(), &self.display) {
                self.ui.handle_event(event);
            }
        }

        set_widgets(params, self.ui.set_widgets(), &mut self.ids);

        let mut target = self.display.draw();

        // Render the `Ui` and then display it on the screen.
        if let Some(primitives) = self.ui.draw_if_changed() {
            self.renderer
                .fill(&self.display, primitives, &self.image_map);
            self.renderer
                .draw(&self.display, &mut target, &self.image_map)
                .unwrap();
        }

        target.finish().unwrap();
    }
}

fn set_widgets(params: Arc<MyParameters>, ref mut ui: conrod::UiCell, ids: &mut Ids) {
    use conrod::{color, widget, Borderable, Colorable, Labelable, Positionable, Sizeable, Widget};

    widget::Canvas::new()
        .color(color::CHARCOAL)
        .border(0.0)
        .set(ids.body, ui);

    //let gain_db = state.get_param(ParamId::GainDb);
    let attack = params.get_attack();

    /*
    let (min, max) = {
        let def = state.get_param_def(ParamId::GainDb);
        (def.min, def.max)
    };
    */

    let label = format!("Attack: {:.3} s", attack);

    if let Some(val) = widget::Slider::new(attack, 0.0, 1.0)
        .w_h(300.0, 50.0)
        .middle_of(ids.body)
        .rgb(0.5, 0.3, 0.6)
        .border(1.0)
        .label(&label)
        .label_color(color::WHITE)
        .set(ids.attack_slider, ui)
    {
        params.set_attack(val);
    }
    /*
    for _click in widget::Button::new()
        .middle_of(ids.body)
        .down_from(ids.gain_slider, 45.0)
        .w_h(200.0, 30.0)
        .color(color::RED)
        .label("click me")
    .set(ids.button, ui) {
        info!("Bing!");
    }
    */
}