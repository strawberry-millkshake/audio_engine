extern crate coreaudio;

use coreaudio::audio_unit::render_callback::{self, data};
use coreaudio::audio_unit::{AudioUnit, IOType, SampleFormat};

use std::time::SystemTime;

mod signal_generator;
mod wav_reader;
use signal_generator::SineWaveGenerator;

fn get_now(now: SystemTime) -> u64 {
    match now.elapsed() {
        Ok(elapsed) => elapsed.as_secs(),
        Err(e) => {
            println!("Error: {e:?}");
            0
        }
    }
}

fn main() -> Result<(), coreaudio::Error> {
    let mut song = wav_reader::wav_file::new(
        "./audio/Pierce The Veil - Collide With The Sky - 11 Stained Glass Eyes And Colorful Tears.wav",
    );

    println!("\n\n");

    let frequency_hz = 440.;
    let volume = 0.15;
    let mut samples = SineWaveGenerator::new(frequency_hz, volume);
    let mut audio_unit = AudioUnit::new(IOType::HalOutput)?;
    let stream_format = audio_unit.output_stream_format()?;
    println!("{:#?}", &stream_format);
    assert!(SampleFormat::F32 == stream_format.sample_format);

    type Args = render_callback::Args<data::NonInterleaved<f32>>;

    // let now = SystemTime::now();
    // let mut second_counter = 0;
    // let mut change_the_freak = true;

    audio_unit.set_render_callback(move |args| {
        let Args {
            num_frames,
            mut data,
            ..
        } = args;

        for i in 0..num_frames {
            // let sample = samples.next().unwrap();

            let sample = song.next().unwrap();

            // // ~~~ SECOND COUNTER ~~~
            // if second_counter != get_now(now) {
            //     second_counter += 1;
            // }

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
