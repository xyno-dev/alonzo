use crate::SAMPLE_RATE;

pub struct Wav<'a> {
    sampled_data: &'a Vec<f32>,
}

impl<'a> Wav<'a> {
    pub fn from(sampled_data: &'a Vec<f32>) -> Self {
        Self { sampled_data }
    }

    pub fn generate(&self) -> Vec<u8> {
        let mut file = Vec::new();

        let raw_data = unsafe {
            std::slice::from_raw_parts(
                self.sampled_data.as_ptr() as *const u8,
                self.sampled_data.len() * 4,
            )
        };

        let sampled_data_size = raw_data.len() as u32;
        let total_file_size = sampled_data_size + 44;

        file.extend_from_slice(&[0x52, 0x49, 0x46, 0x46]);
        file.extend_from_slice(&total_file_size.to_le_bytes());
        file.extend_from_slice(&[0x57, 0x41, 0x56, 0x45]);
        file.extend_from_slice(&[0x66, 0x6D, 0x74, 0x20]);
        file.extend_from_slice(&[0x10, 0x00, 0x00, 0x00]);
        file.extend_from_slice(&[0x03, 0x00]);
        file.extend_from_slice(&[0x01, 0x00]);
        file.extend_from_slice(&(SAMPLE_RATE as u32).to_le_bytes());
        file.extend_from_slice(&((SAMPLE_RATE * 4) as u32).to_le_bytes());
        file.extend_from_slice(&[0x04, 0x00]);
        file.extend_from_slice(&[0x20, 0x00]);
        file.extend_from_slice(&[0x64, 0x61, 0x74, 0x61]);
        file.extend_from_slice(&sampled_data_size.to_le_bytes());
        file.extend_from_slice(raw_data);

        file
    }
}
