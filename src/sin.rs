use std::f32::consts::TAU;

pub struct SinWave {
    frequency: f32,
    amplitude: f32,
    phase: f32,
}
pub struct SineWaveGenerator {
    time: f32,
    sin_wave: SinWave,
}

impl SineWaveGenerator {
    pub fn new(frequency: f32, amplitude: f32, phase: f32) -> Self {
        SineWaveGenerator {
            time: 0.,
            sin_wave: SinWave {
                frequency,
                amplitude,
                phase,
            },
        }
    }
}

impl Iterator for SineWaveGenerator {
    type Item = [f32; 2];
    fn next(&mut self) -> Option<[f32; 2]> {
        self.time += 1. / 44_100.;
        let current_sample =
            ((self.sin_wave.frequency * self.time * TAU).sin() * self.sin_wave.amplitude) as f32;

        Some([current_sample, current_sample])
    }
}
