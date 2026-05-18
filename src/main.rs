//! A basic output stream example, using an Output AudioUnit to generate a sine wave.

extern crate coreaudio;

use coreaudio::audio_unit::render_callback::{self, data};
use coreaudio::audio_unit::{AudioUnit, IOType, SampleFormat};
use std::f64::consts::PI;

use std::time::SystemTime;

fn get_now(now: SystemTime) -> u64 {
    match now.elapsed() {
        Ok(elapsed) => elapsed.as_secs(),
        Err(e) => {
            println!("Error: {e:?}");
            0
        }
    }
}

struct SineWaveGenerator {
    time: f64,
    freq: f64,
    volume: f64,
    exponent: f64,
}

impl SineWaveGenerator {
    fn new(freq: f64, volume: f64) -> Self {
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

fn main() -> Result<(), coreaudio::Error> {
    let frequency_hz = 440.;
    let volume = 0.15;
    let mut samples = SineWaveGenerator::new(frequency_hz, volume);

    let mut audio_unit = AudioUnit::new(IOType::HalOutput)?;

    // Read the input format. This is counterintuitive, but it's the format used when sending
    // audio data to the AudioUnit representing the output device. This is separate from the
    // format the AudioUnit later uses to send the data to the hardware device.
    let stream_format = audio_unit.output_stream_format()?;
    println!("{:#?}", &stream_format);
    // For this example, our sine wave expects `f32` data.
    assert!(SampleFormat::F32 == stream_format.sample_format);

    let now = SystemTime::now();

    type Args = render_callback::Args<data::NonInterleaved<f32>>;

    let mut second_counter = 0;
    // let mut change_the_freak = true;

    audio_unit.set_render_callback(move |args| {
        let Args {
            num_frames,
            mut data,
            ..
        } = args;

        // println!("num frames: {}", num_frames);

        for i in 0..num_frames {
            let sample = samples.next().unwrap();

            if second_counter != get_now(now) {
                second_counter += 1;
            }

            for channel in data.channels_mut() {
                channel[i] = sample;
            }
        }
        Ok(())
    })?;

    audio_unit.start()?;
    loop {}

    // std::thread::sleep(std::time::Duration::from_millis(10_000));
    // Ok(())
}
