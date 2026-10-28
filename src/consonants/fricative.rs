use crate::{SAMPLE_RATE, bump, sine};

pub fn f(frequency: f32, duration: u64) -> Vec<f32> {
    let mut wave: Vec<f32> = sine(frequency, 1.0, duration);

    for i in 2..=20 {
        let harmonic = frequency * (i as f32);
        wave = sine(
            harmonic,
            bump(340.0, harmonic) + bump(1100.0, harmonic) + bump(2080.0, harmonic),
            duration,
        )
        .iter()
        .zip(wave)
        .map(|(x, y)| x + y)
        .collect();
    }

    wave
}

pub fn v(frequency: f32, duration: u64) -> Vec<f32> {
    let mut wave: Vec<f32> = sine(frequency, 1.0, duration);

    for i in 2..=20 {
        let harmonic = frequency * (i as f32);
        wave = sine(
            harmonic,
            bump(220.0, harmonic) + bump(1100.0, harmonic) + bump(2080.0, harmonic),
            duration,
        )
        .iter()
        .zip(wave)
        .map(|(x, y)| x + y)
        .collect();
    }

    wave
}

pub fn s(duration: u64) -> Vec<f32> {
    let mut wave: Vec<f32> = Vec::new();

    for _ in 0..(SAMPLE_RATE / 1000 * duration) {
        wave.push(rand::random_range(-0.3..0.3));
    }

    wave
}
