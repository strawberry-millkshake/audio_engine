use std::fs;

pub struct Wav {
    pub bytes: Vec<u8>,
    sample_rate: u32,
    bits_per_sample: u16,
    index: usize,
}

impl Wav {
    pub fn new(file_path: &str) -> Wav {
        let print_wav_meta_data = false;

        let bytes = fs::read(file_path).expect("Could not read the file");

        // ~~~ HEADER SECTION ~~~

        let header = &bytes[0..4].iter().map(|x| *x as char).collect::<String>();
        let file_size_bytes = u32::from_le_bytes(bytes[4..8].try_into().unwrap());
        let format = bytes[8..12].iter().map(|x| *x as char).collect::<String>();
        assert_eq!(
            file_size_bytes,
            (bytes.len() - 8) as u32,
            "either the file is lying about its size or there's something wrong with the parser"
        );

        // ~~~ FORMAT SECTION ~~~

        let mut i = 0;

        while String::from_utf8_lossy(&bytes[i..i + 4]) != "fmt " {
            i += 1;
        }

        let subchunk_one_id = &bytes[i..i + 4]
            .iter()
            .map(|x| *x as char)
            .collect::<String>();
        let subchunk_one_size = u32::from_le_bytes(bytes[i + 4..i + 8].try_into().unwrap());
        let audio_format = u16::from_le_bytes(bytes[i + 8..i + 10].try_into().unwrap());
        let number_of_channels = u16::from_le_bytes(bytes[i + 10..i + 12].try_into().unwrap());
        let sample_rate = u32::from_le_bytes(bytes[i + 12..i + 16].try_into().unwrap());
        let byte_rate = u32::from_le_bytes(bytes[i + 16..i + 20].try_into().unwrap());
        let block_align = u16::from_le_bytes(bytes[i + 20..i + 22].try_into().unwrap());
        let bits_per_sample = u16::from_le_bytes(bytes[i + 22..i + 24].try_into().unwrap());

        // ~~~ DATA SECTION ~~~

        while String::from_utf8_lossy(&bytes[i..i + 4]) != "data" {
            i += 1;
        }

        let subchunk_two_id = String::from_utf8_lossy(&bytes[i..i + 4]);
        let subchunk_two_size = u32::from_le_bytes(bytes[i + 4..i + 8].try_into().unwrap());
        assert_eq!(
            subchunk_two_id, "data",
            "There is something wrong with this audio file"
        );

        if print_wav_meta_data {
            println!("\n~~~ HEADER SECTION ~~~\n");
            println!("header: {:?}", header);
            println!("file size: {:?}", file_size_bytes);
            println!("format: {:?}", format);

            println!("\n~~~ FORMAT SECTION ~~~\n");
            println!("subchunk 1 id: {:?}", subchunk_one_id);
            println!("subchunk 1 size: {:?}", subchunk_one_size);
            println!("audio format: {:?}", audio_format);
            println!("num channels: {:?}", number_of_channels);
            println!("sample rate: {:?}", sample_rate);
            println!("byte rate: {:?}", byte_rate);
            println!("block align: {:?}", block_align);
            println!("bits per sample {:?}", bits_per_sample);

            println!("\n~~~ DATA SECTION ~~~\n");
            println!("data section header {:?}", subchunk_two_id);
            println!("subchunk two size: {:?}", subchunk_two_size);
        }

        Wav {
            bytes,
            sample_rate,
            bits_per_sample,
            index: 72,
        }
    }
}

impl Iterator for Wav {
    type Item = [f32; 2];
    fn next(&mut self) -> Option<[f32; 2]> {
        let mut current_sample = [0., 0.];

        if self.bits_per_sample == 16 && self.index + 2 < self.bytes.len(){
            let left_channel_sample =
                i16::from_le_bytes(self.bytes[self.index..self.index + 2].try_into().unwrap());
            let right_channel_sample = i16::from_le_bytes(
                self.bytes[self.index + 2..self.index + 4]
                    .try_into()
                    .unwrap(),
            );
            current_sample = [
                (left_channel_sample as f32) / (i16::MAX as f32),
                (right_channel_sample as f32) / (i16::MAX as f32),
            ];
            self.index += 4;
        } else if self.bits_per_sample == 32 && self.index + 4 < self.bytes.len(){
            let left_channel_sample =
                i32::from_le_bytes(self.bytes[self.index..self.index + 4].try_into().unwrap());
            let right_channel_sample = i32::from_le_bytes(
                self.bytes[self.index + 4..self.index + 8]
                    .try_into()
                    .unwrap(),
            );
            current_sample = [
                (left_channel_sample as f32) / (i32::MAX as f32),
                (right_channel_sample as f32) / (i32::MAX as f32),
            ];
            self.index += 8;
        }

        Some(current_sample)
    }
}