use std::{env, fs};
use half::f16;

pub struct wav_file {
    wav_bytes: Vec<u8>,
    sample_rate: u32,
    bits_per_sample: u16,
    index: usize,
}

impl wav_file {
    pub fn new(file_path: &str) -> wav_file {
        let wav_bytes = fs::read(file_path).expect("Could not read the file");

        // ~~~ HEADER SECTION ~~~
        println!("\n~~~ HEADER SECTION ~~~\n");

        let header = &wav_bytes[0..4]
            .iter()
            .map(|x| *x as char)
            .collect::<String>();
        println!("header: {:?}", header);

        let file_size_bytes = u32::from_le_bytes(wav_bytes[4..8].try_into().unwrap());
        println!("file size: {:?}", file_size_bytes);

        assert_eq!(
            file_size_bytes,
            (wav_bytes.len() - 8) as u32,
            "either the file is lying about its size or there's something wrong with the parser"
        );

        let format = wav_bytes[8..12]
            .iter()
            .map(|x| *x as char)
            .collect::<String>();
        println!("format: {:?}", format);

        // ~~~ FORMAT SECTION ~~~
        println!("\n~~~ FORMAT SECTION ~~~\n");

        let subchunk_one_id = &wav_bytes[12..16]
            .iter()
            .map(|x| *x as char)
            .collect::<String>();
        println!("subchunk 1 id: {:?}", subchunk_one_id);

        let subchunk_one_size = u32::from_le_bytes(wav_bytes[16..20].try_into().unwrap());
        println!("subchunk 1 size: {:?}", subchunk_one_size);

        let audio_format = u16::from_le_bytes(wav_bytes[20..22].try_into().unwrap());
        println!("audio format: {:?}", audio_format);

        let number_of_channels = u16::from_le_bytes(wav_bytes[22..24].try_into().unwrap());
        println!("num channels: {:?}", number_of_channels);

        let sample_rate = u32::from_le_bytes(wav_bytes[24..28].try_into().unwrap());
        println!("sample rate: {:?}", sample_rate);

        let byte_rate = u32::from_le_bytes(wav_bytes[28..32].try_into().unwrap());
        println!("byte rate: {:?}", byte_rate);

        let block_align = u16::from_le_bytes(wav_bytes[32..34].try_into().unwrap());
        println!("block align: {:?}", block_align);

        let bits_per_sample = u16::from_le_bytes(wav_bytes[34..36].try_into().unwrap());
        println!("bits per sample {:?}", bits_per_sample);

        // ~~~ DATA SECTION ~~~
        println!("\n~~~ DATA SECTION ~~~\n");

        let subchunk_two_id = String::from_utf8_lossy(&wav_bytes[36..40]);
        println!("data section header {:?}", subchunk_two_id);

        let subchunk_two_size = u32::from_le_bytes(wav_bytes[40..44].try_into().unwrap());
        println!("subchunk two size: {:?}", subchunk_two_size);

        wav_file {
            wav_bytes,
            sample_rate,
            bits_per_sample,
            index: 44,
        }
    }
}


impl Iterator for wav_file {
    type Item = f32;
    fn next(&mut self) -> Option<f32> {
        let sample_int = i16::from_le_bytes(
            self.wav_bytes[self.index..self.index + 2]
                .try_into()
                .unwrap(),
        );
        self.index += 4;

        let output = (sample_int as f32)/(i16::MAX as f32);

        println!("{}", output);

        Some(output)
    }
}
