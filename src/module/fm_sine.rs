use std::f64;

use mixlab_protocol::{FmSineParams, LineType, Terminal};

use crate::engine::{Sample, SAMPLE_RATE, CHANNELS, ZERO_BUFFER_STEREO};
use crate::module::ModuleT;

#[derive(Debug)]
pub struct FmSine {
    params: FmSineParams,
    inputs: Vec<Terminal>,
    outputs: Vec<Terminal>,
}

impl ModuleT for FmSine {
    type Params = FmSineParams;
    type Indication = ();

    fn create(params: Self::Params) -> (Self, Self::Indication) {
        (Self {
            params,
            inputs: vec![LineType::Mono.unlabeled()],
            outputs: vec![LineType::Stereo.unlabeled()],
        }, ())
    }

    fn params(&self) -> Self::Params {
        self.params.clone()
    }

    fn update(&mut self, new_params: Self::Params) -> Option<Self::Indication> {
        self.params = new_params;
        None
    }

    fn run_tick(&mut self, t: u64, inputs: &[Option<&[Sample]>], outputs: &mut [&mut [Sample]]) -> Option<Self::Indication> {
        let len = outputs[0].len() / CHANNELS;

        let input = inputs[0].unwrap_or(&ZERO_BUFFER_STEREO);

        let freq_amp = (self.params.freq_hi - self.params.freq_lo) / 2.0;
        let freq_mid = self.params.freq_lo + freq_amp;

        for i in 0..len {
            let t = (t + i as u64) as f64 / SAMPLE_RATE as f64;
            let co = (freq_mid + freq_amp * input[i] as f64) * 2.0 * f64::consts::PI;
            let x = f64::sin(co * t);

            for chan in 0..CHANNELS {
                outputs[0][i * CHANNELS + chan] = x as Sample;
            }
        }

        None
    }

    fn inputs(&self) -> &[Terminal] {
        &self.inputs
    }

    fn outputs(&self)-> &[Terminal] {
        &self.outputs
    }
}
