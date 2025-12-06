use crate::{
    interface::{ParamType, SoundAlgorithm, SoundParameter},
    utils::PerlinSequence1D,
};

pub struct SimplexOscillator2 {
    sample_rate: u32,
    phase: f32,
    debug_rollover_flag: bool,

    frequency: f32,
    radius: f32,
    harmonics: i32,
    seed: i32,
}

impl SimplexOscillator2 {
    pub fn new() -> Self {
        Self {
            sample_rate: 0,
            phase: 0.0,
            debug_rollover_flag: false,

            frequency: 220.0,
            radius: 1.0,
            harmonics: 1,
            seed: 0,
        }
    }
}

impl SoundAlgorithm for SimplexOscillator2 {
    fn get_name(&self) -> &'static str {
        "Simplex harmonics"
    }

    fn debug_get_freq(&mut self) -> f32 {
        self.frequency
    }

    fn debug_get_and_clear_cycle_flag(&mut self) -> bool {
        let flag = self.debug_rollover_flag;
        self.debug_rollover_flag = false;
        flag
    }

    fn set_sample_rate(&mut self, sample_rate: u32) {
        self.sample_rate = sample_rate;
    }

    fn generate_sample(&mut self) -> f32 {
        debug_assert_ne!(self.sample_rate, 0);
        self.phase += self.frequency / self.sample_rate as f32;

        if self.phase >= 1.0 {
            self.phase -= 1.0;
            self.debug_rollover_flag = true;
        }

        let x = (self.phase * 2.0 * std::f32::consts::PI).sin() * self.radius;
        let y = (self.phase * 2.0 * std::f32::consts::PI).sin() * self.radius;

        let mut output = 0.0;
        for i in 0..self.harmonics {
            let seed = self.seed + 100 * i;
            let x = x * i as f32;
            let y = y * i as f32;
            output += opensimplex2::fast::noise3_ImproveXY(seed.into(), x.into(), y.into(), 0.0)
                / (i + 1) as f32;
        }

        output
    }

    fn parameters(&self) -> Vec<SoundParameter> {
        vec![
            SoundParameter {
                value: self.frequency,
                name: "Frequency",
                param_type: ParamType::Float {
                    min: 22.0,
                    max: 880.0,
                },
            },
            SoundParameter {
                value: self.radius,
                name: "Radius",
                param_type: ParamType::Float {
                    min: 0.0,
                    max: 10.0,
                },
            },
            SoundParameter {
                value: self.radius,
                name: "Harmonics",
                param_type: ParamType::Float {
                    min: 1.0,
                    max: 10.0,
                },
            },
            SoundParameter {
                value: self.seed as f32,
                name: "Seed",
                param_type: ParamType::Float {
                    min: 0.0,
                    max: 10.0,
                },
            },
        ]
    }

    fn update_parameter(&mut self, name: &str, value: f32) {
        match name {
            "Frequency" => {
                self.frequency = value;
            }
            "Radius" => {
                self.radius = value;
            }
            "Harmonics" => {
                self.harmonics = value.round() as i32;
            }
            "Seed" => {
                self.seed = value.round() as i32;
            }
            _ => panic!("Unexpected parameter name: {}", name),
        }
    }
}
