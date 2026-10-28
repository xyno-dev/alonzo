use crate::{bump, sine};

pub fn r(frequency: f32, duration: u64) -> Vec<f32> {
    let mut wave: Vec<f32> = sine(frequency, 1.0, duration);

    for i in 2..=20 {
        let harmonic = frequency * (i as f32);
        wave = sine(
            harmonic,
            bump(310.0, harmonic) + bump(1060.0, harmonic) + bump(1380.0, harmonic),
            duration,
        )
        .iter()
        .zip(wave)
        .map(|(x, y)| x + y)
        .collect();
    }

    wave
}

pub fn l(frequency: f32, duration: u64) -> Vec<f32> {
    let mut wave: Vec<f32> = sine(frequency, 1.0, duration);

    for i in 2..=20 {
        let harmonic = frequency * (i as f32);
        wave = sine(
            harmonic,
            bump(310.0, harmonic) + bump(1050.0, harmonic) + bump(2280.0, harmonic),
            duration,
        )
        .iter()
        .zip(wave)
        .map(|(x, y)| x + y)
        .collect();
    }

    wave
}
