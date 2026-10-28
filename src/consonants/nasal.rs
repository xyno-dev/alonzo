use crate::{bump, sine};

pub fn m(frequency: f32, duration: u64) -> Vec<f32> {
    let mut wave: Vec<f32> = sine(frequency, 1.0, duration);

    sine(270.0, 0.05, duration);
    sine(450.0, 0.05, duration);
    for i in 2..=20 {
        let harmonic = frequency * (i as f32);
        wave = sine(
            harmonic,
            bump(480.0, harmonic) + bump(1270.0, harmonic) + bump(2130.0, harmonic),
            duration,
        )
        .iter()
        .zip(wave)
        .map(|(x, y)| x + y)
        .collect();
    }

    wave
}
