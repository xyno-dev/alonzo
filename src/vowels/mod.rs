use crate::{bump, sine};

pub fn ah(frequency: f32, duration: u64) -> Vec<f32> {
    let mut wave: Vec<f32> = sine(frequency, 1.0, duration);

    for i in 2..=20 {
        let harmonic = frequency * (i as f32);
        wave = sine(
            harmonic,
            bump(700.0, harmonic) + bump(1220.0, harmonic) + bump(2600.0, harmonic),
            duration,
        )
        .iter()
        .zip(wave)
        .map(|(x, y)| x + y)
        .collect();
    }

    wave
}

pub fn ee(frequency: f32, duration: u64) -> Vec<f32> {
    let mut wave: Vec<f32> = sine(frequency, 1.0, duration);

    for i in 2..=20 {
        let harmonic = frequency * (i as f32);
        wave = sine(
            harmonic,
            bump(270.0, harmonic) + bump(2300.0, harmonic) + bump(3000.0, harmonic),
            duration,
        )
        .iter()
        .zip(wave)
        .map(|(x, y)| x + y)
        .collect();
    }

    wave
}

pub fn ih(frequency: f32, duration: u64) -> Vec<f32> {
    let mut wave: Vec<f32> = sine(frequency, 1.0, duration);

    for i in 2..=20 {
        let harmonic = frequency * (i as f32);
        wave = sine(
            harmonic,
            bump(400.0, harmonic) + bump(1800.0, harmonic) + bump(2570.0, harmonic),
            duration,
        )
        .iter()
        .zip(wave)
        .map(|(x, y)| x + y)
        .collect();
    }

    wave
}

pub fn oo(frequency: f32, duration: u64) -> Vec<f32> {
    let mut wave: Vec<f32> = sine(frequency, 1.0, duration);

    for i in 2..=20 {
        let harmonic = frequency * (i as f32);
        wave = sine(
            harmonic,
            bump(320.0, harmonic) + bump(900.0, harmonic) + bump(2200.0, harmonic),
            duration,
        )
        .iter()
        .zip(wave)
        .map(|(x, y)| x + y)
        .collect();
    }

    wave
}

pub fn e(frequency: f32, duration: u64) -> Vec<f32> {
    let mut wave: Vec<f32> = sine(frequency, 1.0, duration);

    for i in 2..=20 {
        let harmonic = frequency * (i as f32);
        wave = sine(
            harmonic,
            bump(330.0, harmonic) + bump(2020.0, harmonic) + bump(2300.0, harmonic),
            duration,
        )
        .iter()
        .zip(wave)
        .map(|(x, y)| x + y)
        .collect();
    }

    wave
}

pub fn o(frequency: f32, duration: u64) -> Vec<f32> {
    let mut wave: Vec<f32> = sine(frequency, 1.0, duration);

    for i in 2..=20 {
        let harmonic = frequency * (i as f32);
        wave = sine(
            harmonic,
            bump(540.0, harmonic) + bump(1100.0, harmonic) + bump(2300.0, harmonic),
            duration,
        )
        .iter()
        .zip(wave)
        .map(|(x, y)| x + y)
        .collect();
    }

    wave
}

pub fn a(frequency: f32, duration: u64) -> Vec<f32> {
    let mut wave: Vec<f32> = sine(frequency, 1.0, duration);

    for i in 2..=20 {
        let harmonic = frequency * (i as f32);
        wave = sine(
            harmonic,
            bump(620.0, harmonic) + bump(1660.0, harmonic) + bump(2430.0, harmonic),
            duration,
        )
        .iter()
        .zip(wave)
        .map(|(x, y)| x + y)
        .collect();
    }

    wave
}

pub fn uh(frequency: f32, duration: u64) -> Vec<f32> {
    let mut wave: Vec<f32> = sine(frequency, 1.0, duration);

    for i in 2..=20 {
        let harmonic = frequency * (i as f32);
        wave = sine(
            harmonic,
            bump(620.0, harmonic) + bump(1200.0, harmonic) + bump(2150.0, harmonic),
            duration,
        )
        .iter()
        .zip(wave)
        .map(|(x, y)| x + y)
        .collect();
    }

    wave
}
