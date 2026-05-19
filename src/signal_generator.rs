use std::f64::consts::PI;

pub struct SineWaveGenerator {
    time: f64,
    freq: f64,
    volume: f64,
    exponent: f64,
}

impl SineWaveGenerator {
    pub fn new(freq: f64, volume: f64) -> Self {
        SineWaveGenerator {
            time: 0.,
            freq,
            volume,
            exponent: 10.,
        }
    }
}

impl Iterator for SineWaveGenerator {
    type Item = f32;
    fn next(&mut self) -> Option<f32> {
        self.time += 1. / 44_100.;
        self.exponent = self.time;
        let output =
            ((self.freq * self.time * PI * 2.).sin() * self.volume) as f32;

        Some(output)
    }
}