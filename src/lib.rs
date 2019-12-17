mod editor;
use editor::MyEditor;
use editor::MyEditorBuilder;
use vst::plugin::HostCallback;
use vst::util::ParameterTransfer;

use sample::Sample;

use hound::WavReader;
//mod ui;

mod parameter;
use parameter::*;

mod ui;

use vst::plugin_main;

use std::sync::Arc;

use vst::api::{Events, Supported};
use vst::buffer::AudioBuffer;
use vst::editor::Editor;
use vst::event::Event;
use vst::plugin::{CanDo, Category, Info, Plugin, PluginParameters};

use std::f64::consts::PI;

const TAU: f64 = PI * 2.0;

fn midi_pitch_to_freq(pitch: u8) -> f64 {
    const A4_PITCH: i8 = 69;
    const A4_FREQ: f64 = 440.0;

    ((f64::from(pitch as i8 - A4_PITCH)) / 12.0).exp2() * A4_FREQ
}

pub enum Index {
    Attack,
}

//#[derive(Clone)]
struct MyPlugin {
    sampling_rate: f64,
    time: f64,
    note_duration: f64,
    note: Option<u8>,
    params: Arc<MyParameters>,
    editor_exists: bool,
    audio_source: &'static [u8],
    signal: Option<Box<dyn sample::Signal<Frame = [f32; 1]>>>,
    ring_buffer: sample::ring_buffer::Fixed,
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
        self.note = Some(note);

        let reader = WavReader::new(self.audio_source.as_ref()).unwrap();
        use sample::{interpolate, ring_buffer, signal, Sample, Signal};

        let spec = reader.spec();
        let mut target = spec;
        target.sample_rate = 44100 as u32;

        let samples = reader.into_samples::<f32>().filter_map(Result::ok);
        //.map(f32::to_sample::<f32>);

        let signal = signal::from_interleaved_samples_iter(samples);

        let ring_buffer = ring_buffer::Fixed::from([[0.0 as f32]; 64]);
        let sinc = interpolate::Sinc::new(ring_buffer);
        let signal = signal.from_hz_to_hz(sinc, spec.sample_rate as f64, target.sample_rate as f64);

        self.signal = Some(Box::new(signal));
    }

    fn note_off(&mut self, note: u8) {
        if self.note == Some(note) {
            self.note = None
        }
        if let Some(_) = self.signal {
            self.signal = None
        }
    }
}

impl Default for MyPlugin {
    fn default() -> MyPlugin {
        let source = include_bytes!("../resource/audio/audio.wav");

        MyPlugin {
            sampling_rate: 44100.0,
            note_duration: 0.0,
            time: 0.0,
            note: None,
            params: Arc::new(MyParameters::default()),
            editor_exists: false,
            audio_source: source,
            signal: None,
        }
    }
}

const PARAMETER_COUNT: usize = 1;

impl Plugin for MyPlugin {
    fn new(host: HostCallback) -> Self {
        MyPlugin {
            params: Arc::new(
                MyParametersBuilder::default()
                    .host(host)
                    .transfer(ParameterTransfer::new(PARAMETER_COUNT))
                    .build()
                    .unwrap(),
            ),
            ..Default::default()
        }
    }

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
        match self.editor_exists {
            false => {
                let editor = MyEditorBuilder::default()
                    .params(Arc::clone(&self.params))
                    .build()
                    .unwrap();

                Some(Box::new(editor))
            }
            true => None,
        }
    }

    fn get_parameter_object(&mut self) -> Arc<dyn PluginParameters> {
        Arc::clone(&self.params) as Arc<dyn PluginParameters>
    }

    fn process_events(&mut self, events: &Events) {
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
        let (_, mut outputs) = buffer.split();
        let output_count = outputs.len();
        let per_sample = self.time_per_sample();
        let mut output_sample;

        let attack = self.params.get_parameter(Index::Attack as i32);

        for sample_index in 0..samples {
            let mut time = self.time;
            let mut note_duration = self.note_duration;

            /*
            if let Some(current_note) = self.note {
                let signal = (time * midi_pitch_to_freq(current_note) * TAU).sin();
                let attack = f64::from(attack);
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
            }*/

            //simple_logging::log_to_file("M:/VST2/x64/Kazzix/log.txt", log::LevelFilter::Trace);
            //use log::info;

            if let Some(signal) = &mut self.signal {
                output_sample = signal.next()[0];
            } else {
                output_sample = 0.0;
            }

            //info!("output :{}", output_sample);

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
