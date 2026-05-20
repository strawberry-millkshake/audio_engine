use std::fs;

pub struct Wav {
    bytes: Vec<u8>,
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

        let subchunk_one_id = &bytes[12..16].iter().map(|x| *x as char).collect::<String>();
        let subchunk_one_size = u32::from_le_bytes(bytes[16..20].try_into().unwrap());
        let audio_format = u16::from_le_bytes(bytes[20..22].try_into().unwrap());
        let number_of_channels = u16::from_le_bytes(bytes[22..24].try_into().unwrap());
        let sample_rate = u32::from_le_bytes(bytes[24..28].try_into().unwrap());
        let byte_rate = u32::from_le_bytes(bytes[28..32].try_into().unwrap());
        let block_align = u16::from_le_bytes(bytes[32..34].try_into().unwrap());
        let bits_per_sample = u16::from_le_bytes(bytes[34..36].try_into().unwrap());
        assert_eq!(
            bits_per_sample, 16,
            "this wav file isn't 16 bits per sample and you need to go impliment the new format"
        );

        // ~~~ DATA SECTION ~~~

        let subchunk_two_id = String::from_utf8_lossy(&bytes[36..40]);
        let subchunk_two_size = u32::from_le_bytes(bytes[40..44].try_into().unwrap());
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
            index: 44,
        }
    }
}

impl Iterator for Wav {
    type Item = [f32; 2];
    fn next(&mut self) -> Option<[f32; 2]> {
        let left_channel_sample =
            i16::from_le_bytes(self.bytes[self.index..self.index + 2].try_into().unwrap());

        let right_channel_sample = i16::from_le_bytes(
            self.bytes[self.index + 2..self.index + 4]
                .try_into()
                .unwrap(),
        );
        self.index += 4;

        let current_sample = [
            (left_channel_sample as f32) / (i16::MAX as f32),
            (right_channel_sample as f32) / (i16::MAX as f32),
        ];

        Some(current_sample)
    }
}
