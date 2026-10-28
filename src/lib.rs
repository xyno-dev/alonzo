use std::f32::consts::{E, PI};

pub mod consonants;
pub mod vowels;

pub const SAMPLE_RATE: u64 = 40000;

pub fn sine(frequency: f32, amplitude: f32, duration: u64) -> Vec<f32> {
    let wave: Vec<f32> = (0..(SAMPLE_RATE / 1000 * duration))
        .map(|x| x as f32)
        .map(|x| x * ((2.0 * PI * frequency) / SAMPLE_RATE as f32))
        .map(|x| x.sin())
        .map(|x| x * amplitude)
        .collect();

    wave
}

fn bump(center: f32, x: f32) -> f32 {
    let peak = 1.0;
    let width = 125.0;

    println!(
        "{center}: {}",
        peak * E.powf((-1.0 * (x - center).powi(2)) / (width * width * 2.0))
    );

    peak * E.powf((-1.0 * (x - center).powi(2)) / (width * width * 2.0))
}
