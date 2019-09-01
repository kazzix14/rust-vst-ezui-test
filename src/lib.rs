mod editor;
use editor::MyEditor;
use editor::MyEditorBuilder;

use conrod::glium;
use conrod::glium::DisplayBuild;
use conrod::glium::Surface;
use conrod::widget_ids;

use derive_builder::Builder;
use log;

use vst::plugin_main;

use std::sync::Arc;

use vst::api::{Events, Supported};
use vst::buffer::AudioBuffer;
use vst::editor::Editor;
use vst::event::Event;
use vst::plugin::{CanDo, Category, Info, Plugin, PluginParameters};
use vst::util::AtomicFloat;

use std::f64::consts::PI;
use std::os::raw::c_void;

use std::path::{Path, PathBuf};

fn midi_pitch_to_freq(pitch: u8) -> f64 {
    const A4_PITCH: i8 = 69;
    const A4_FREQ: f64 = 440.0;

    ((f64::from(pitch as i8 - A4_PITCH)) / 12.0).exp2() * A4_FREQ
}

widget_ids! {
    struct Ids{
        body,
        chick,
        attack_slider,
    }
}

struct UiState {
    display: glium::Display,
    ui: conrod::Ui,
    image_map: conrod::image::Map<glium::texture::Texture2d>,
    ids: Ids,
    renderer: conrod::backend::glium::Renderer,
}

#[derive(Clone)]
struct MyPlugin {
    sampling_rate: f64,
    time: f64,
    note_duration: f64,
    note: Option<u8>,
    params: Arc<MyParameters>,
    editor_returned: bool,
}

impl UiState {
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

        UiState {
            display: display,
            ui: ui,
            image_map: image_map,
            renderer: renderer,
            ids: ids,
        }
    }

    fn draw(&mut self, params: Arc<MyParameters>) {
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


struct MyParameters {
    attack: AtomicFloat,
}

impl Default for MyParameters {
    fn default() -> MyParameters {
        MyParameters {
            attack: AtomicFloat::new(0.5),
        }
    }
}

impl MyParameters {
    pub fn set_attack(&self, value: f32) {
        self.attack.set(value)
    }

    pub fn get_attack(&self) -> f32 {
        self.attack.get()
    }
}

impl PluginParameters for MyParameters {
    fn get_parameter(&self, index: i32) -> f32 {
        match index {
            0 => self.get_attack(),
            _ => 0.0,
        }
    }

    fn set_parameter(&self, index: i32, value: f32) {
        match index {
            0 => self.set_attack(value),
            _ => (),
        }
    }

    fn get_parameter_name(&self, index: i32) -> String {
        match index {
            0 => "attack".to_string(),
            _ => "".to_string(),
        }
    }

    fn get_parameter_label(&self, index: i32) -> String {
        match index {
            0 => "s".to_string(),
            _ => "".to_string(),
        }
    }

    fn get_parameter_text(&self, index: i32) -> String {
        match index {
            0 => format!("{:.3}", self.get_attack()),
            _ => format!(""),
        }
    }
}

impl MyPlugin {
    fn time_per_sample(&self) -> f64 {
        1.0 / self.sampling_rate
    }

    fn process_midi_event(&mut self, data: [u8; 3]) {
        match data[0] & 0xf0 {
            0x80 => self.note_off(data[1]),
            0x90 => match data[2] {
                0x00 => self.note_off(data[1]),
                _ => self.note_on(data[1]),
            },
            _ => (),
        }
    }

    fn note_on(&mut self, note: u8) {
        self.note_duration = 0.0;
        self.note = Some(note)
    }

    fn note_off(&mut self, note: u8) {
        if self.note == Some(note) {
            self.note = None
        }
    }
}

pub const TAU: f64 = PI * 2.0;

impl Default for MyPlugin {
    fn default() -> MyPlugin {
        MyPlugin {
            sampling_rate: 44100.0,
            note_duration: 0.0,
            time: 0.0,
            note: None,
            params: Arc::new(MyParameters::default()),
            editor_returned: false,
        }
    }
}

impl Plugin for MyPlugin {
    fn get_info(&self) -> Info {
        Info {
            name: "Vass 0".to_string(),
            vendor: "Kazzix".to_string(),
            unique_id: 1470,
            inputs: 2,
            outputs: 2,
            parameters: 1,
            category: Category::Synth,

            ..Default::default()
        }
    }

    fn get_editor(&mut self) -> Option<Box<dyn Editor>> {
        match self.editor_returned {
            false => {
                self.editor_returned = true;
                Some(Box::new(
                    MyEditorBuilder::default()
                        .params(Arc::clone(&self.params))
                        .build()
                        .unwrap(),
                ))
            }
            true => None,
        }
    }

    fn get_parameter_object(&mut self) -> Arc<dyn PluginParameters> {
        Arc::clone(&self.params) as Arc<dyn PluginParameters>
    }

    fn process_events(&mut self, events: &Events) {
        println!("event!");
        for event in events.events() {
            match event {
                Event::Midi(ev) => self.process_midi_event(ev.data),
                _ => (),
            }
        }
    }

    fn set_sample_rate(&mut self, rate: f32) {
        self.sampling_rate = f64::from(rate);
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        let samples = buffer.samples();
        let (_, outputs) = buffer.split();
        let output_count = outputs.len();
        let per_sample = self.time_per_sample();
        let mut output_sample;

        for sample_index in 0..samples {
            let mut time = self.time;
            let mut note_duration = self.note_duration;
            if let Some(current_note) = self.note {
                let signal = (time * midi_pitch_to_freq(current_note) * TAU).sin();
                let attack = f64::from(self.params.get_attack());
                let alpha = if note_duration < attack {
                    note_duration / attack
                } else {
                    1.0
                };

                output_sample = (signal * alpha) as f32;

                self.time += per_sample;
                self.note_duration += per_sample;
            } else {
                output_sample = 0.0;
            }

            for buff_index in 0..output_count {
                let buff = outputs.get_mut(buff_index);
                buff[sample_index] = output_sample;
            }
        }
    }

    fn can_do(&self, can_do: CanDo) -> Supported {
        match can_do {
            CanDo::ReceiveMidiEvent => Supported::Yes,
            _ => Supported::Maybe,
        }
    }
}

plugin_main!(MyPlugin);
