extern crate coreaudio;

use coreaudio::audio_unit::audio_format::LinearPcmFlags;
use coreaudio::audio_unit::macos_helpers::{audio_unit_from_device_id, get_default_device_id};
use coreaudio::audio_unit::render_callback::{self, data};
use coreaudio::audio_unit::{Element, SampleFormat, Scope, StreamFormat};
use objc2_audio_toolbox::kAudioUnitProperty_StreamFormat;

mod sin;
mod time_utils;
mod wav_reader;

fn main() -> Result<(), coreaudio::Error> {
    // ~~~ CORE AUDIO SETUP ~~~

    let stream_format = StreamFormat {
        sample_rate: 44100.,
        sample_format: SampleFormat::F32,
        flags: LinearPcmFlags::IS_FLOAT | LinearPcmFlags::IS_PACKED,
        channels: 2,
    };
    let audio_unit_id = get_default_device_id(false).unwrap();
    let mut audio_unit = audio_unit_from_device_id(audio_unit_id, false)?;
    let id = kAudioUnitProperty_StreamFormat;
    let stream_format_asbd = stream_format.to_asbd();
    audio_unit.set_property(id, Scope::Input, Element::Output, Some(&stream_format_asbd))?;
    let stream_format = audio_unit.output_stream_format()?;
    assert_eq!(
        stream_format.sample_format,
        SampleFormat::F32,
        "plz use float"
    );
    assert_eq!(stream_format.sample_rate, 44100., "plz use 441k");
    type Args = render_callback::Args<data::Interleaved<f32>>;

    // ~~~ OTHER SETUP ~~~
    let mut song = wav_reader::Wav::new(
        "./audio/Pierce The Veil - Collide With The Sky - 11 Stained Glass Eyes And Colorful Tears.wav",
    );
    let frequency = 440.;
    let amplitude = 0.15;
    let phase = 1.0;
    let mut sin_wav = sin::SineWaveGenerator::new(frequency, amplitude, phase);

    // ~~~ MAIN LOOP ~~~

    audio_unit.set_render_callback(move |args| {
        let Args {
            num_frames, data, ..
        } = args;

        for current_frame_index in 0..num_frames {
            let index_left = 2 * current_frame_index;
            let index_right = (2 * current_frame_index) + 1;

            let sin_samples = sin_wav.next().unwrap();
            let song_samples = song.next().unwrap();

            data.buffer[index_left] = song_samples[0];
            data.buffer[index_right] = song_samples[1];
        }
        Ok(())
    })?;

    audio_unit.start()?;
    loop {}
    // std::thread::sleep(std::time::Duration::from_millis(10_000));
    // Ok(())
}
