use vst::plugin::HostCallback;
use vst::plugin::PluginParameters;
use vst::util::AtomicFloat;
use vst::util::ParameterTransfer;

use derive_builder::Builder;

#[derive(Default, Builder)]
#[builder(pattern = "owned")]
pub struct MyParameters {
    //attack: AtomicFloat,
    host: HostCallback,
    transfer: ParameterTransfer,
}

impl PluginParameters for MyParameters {
    fn get_parameter(&self, index: i32) -> f32 {
        self.transfer.get_parameter(index as usize)
    }

    fn set_parameter(&self, index: i32, value: f32) {
        self.transfer.set_parameter(index as usize, value)
    }
    /*
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
    */
}
