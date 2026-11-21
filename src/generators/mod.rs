use crate::{SAMPLE_RATE, bump, sine};

pub struct AdditiveGenerator {
    formant1: f32,
    formant2: f32,
    formant3: f32,
}

impl AdditiveGenerator {
    pub fn new(formant1: f32, formant2: f32, formant3: f32) -> Self {
        Self {
            formant1,
            formant2,
            formant3,
        }
    }

    pub fn generate(&self, base: f32, duration: u64) -> Vec<f32> {
        let formant1 = self.formant1;
        let formant2 = self.formant2;
        let formant3 = self.formant3;

        let mut wave: Vec<f32> = sine(base, 1.0, duration);

        for i in 2..=20 {
            let harmonic = base * (i as f32);
            wave = sine(
                harmonic,
                bump(formant1, harmonic) + bump(formant2, harmonic) + bump(formant3, harmonic),
                duration,
            )
            .iter()
            .zip(wave)
            .map(|(x, y)| x + y)
            .collect();
        }

        wave
    }
}

pub struct ExtendedAdditiveGenerator {
    frequency1: f32,
    frequency2: f32,
    volume1: f32,
    volume2: f32,
    formant1: f32,
    formant2: f32,
    formant3: f32,
}

impl ExtendedAdditiveGenerator {
    pub fn new(
        frequency1: f32,
        frequency2: f32,
        volume1: f32,
        volume2: f32,
        formant1: f32,
        formant2: f32,
        formant3: f32,
    ) -> Self {
        Self {
            frequency1,
            frequency2,
            volume1,
            volume2,
            formant1,
            formant2,
            formant3,
        }
    }

    pub fn generate(&self, base: f32, duration: u64) -> Vec<f32> {
        let frequency1 = self.frequency1;
        let frequency2 = self.frequency2;
        let volume1 = self.volume1;
        let volume2 = self.volume2;
        let formant1 = self.formant1;
        let formant2 = self.formant2;
        let formant3 = self.formant3;

        let mut wave: Vec<f32> = sine(base, 1.0, duration);

        wave = sine(frequency1, volume1, duration)
            .iter()
            .zip(wave)
            .map(|(x, y)| x + y)
            .collect();
        wave = sine(frequency2, volume2, duration)
            .iter()
            .zip(wave)
            .map(|(x, y)| x + y)
            .collect();

        for i in 2..=20 {
            let harmonic = base * (i as f32);
            wave = sine(
                harmonic,
                bump(formant1, harmonic) + bump(formant2, harmonic) + bump(formant3, harmonic),
                duration,
            )
            .iter()
            .zip(wave)
            .map(|(x, y)| x + y)
            .collect();
        }

        wave
    }
}

pub struct NoiseGenerator();

impl NoiseGenerator {
    pub fn new() -> Self {
        Self()
    }

    pub fn generate(&self, duration: u64) -> Vec<f32> {
        let mut wave: Vec<f32> = Vec::new();

        for _ in 0..(SAMPLE_RATE / 1000 * duration) {
            wave.push(rand::random_range(-0.3..0.3));
        }

        wave
    }
}

pub struct PlosiveGenerator {
    frequency1: f32,
    frequency2: f32,
    frequency3: f32,
    volume1: f32,
    volume2: f32,
    volume3: f32,
}

impl PlosiveGenerator {
    pub fn new(
        frequency1: f32,
        frequency2: f32,
        frequency3: f32,
        volume1: f32,
        volume2: f32,
        volume3: f32,
    ) -> Self {
        Self {
            frequency1,
            frequency2,
            frequency3,
            volume1,
            volume2,
            volume3,
        }
    }

    pub fn generate(&self, duration: u64) -> Vec<f32> {
        let frequency1 = self.frequency1;
        let frequency2 = self.frequency2;
        let frequency3 = self.frequency3;
        let volume1 = self.volume1;
        let volume2 = self.volume2;
        let volume3 = self.volume3;

        let mut wave = sine(frequency1, volume1, duration);

        wave = sine(frequency2, volume2, duration)
            .iter()
            .zip(wave)
            .map(|(x, y)| x + y)
            .collect();
        wave = sine(frequency3, volume3, duration)
            .iter()
            .zip(wave)
            .map(|(x, y)| x + y)
            .collect();

        wave
    }
}
