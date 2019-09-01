use vst::util::AtomicFloat;
use vst::plugin::PluginParameters;

pub struct MyParameters {
    attack: AtomicFloat,
}

impl MyParameters {
    pub fn set_attack(&self, value: f32) {
        self.attack.set(value)
    }

    pub fn get_attack(&self) -> f32 {
        self.attack.get()
    }
}

impl Default for MyParameters {
    fn default() -> MyParameters {
        MyParameters {
            attack: AtomicFloat::new(0.5),
        }
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