use rodio::{
    buffer::SamplesBuffer, mixer::Mixer, source::SineWave, ChannelCount, SampleRate, Source
};
use std::{f32, thread, time::Duration};

fn sine(freq: f32, volume: f32, duration: u64, mixer: &Mixer) {
    let wave = SineWave::new(freq)
        .amplify(volume)
        .take_duration(Duration::from_millis(duration));
    mixer.add(wave);
}

// Gaussian bump function :D
fn bump(center: f32, x: f32) -> f32 {
    let peak = 1.0;
    let width = 100.0;

    println!(
        "{}",
        peak * f32::consts::E.powf((-1.0 * (x - center)) / (width))
    );

    println!("##########{}", (x - center).powf(0.8));

    peak * f32::consts::E.powf((-1.0 * (x - center).powi(2)) / (width * width * 2.0))
}

fn ah(frequency: f32, duration: u64, mixer: &Mixer) {
    sine(frequency, 1.0, duration, mixer);
    for i in 2..=20 {
        let harmonic = frequency * (i as f32);
        sine(
            harmonic,
            bump(190.0, harmonic) + bump(750.0, harmonic) + bump(940.0, harmonic),
            duration,
            mixer,
        );
    }

    thread::sleep(Duration::from_millis(duration));
}

fn ee(frequency: f32, duration: u64, mixer: &Mixer) {
    sine(frequency, 1.0, duration, mixer);
    for i in 2..=20 {
        let harmonic = frequency * (i as f32);
        sine(
            harmonic,
            bump(270.0, harmonic) + bump(2160.0, harmonic) + bump(2400.0, harmonic),
            duration,
            mixer,
        );
    }

    thread::sleep(Duration::from_millis(duration));
}

fn ih(frequency: f32, duration: u64, mixer: &Mixer) {
    sine(frequency, 1.0, duration, mixer);
    for i in 2..=20 {
        let harmonic = frequency * (i as f32);
        sine(
            harmonic,
            bump(400.0, harmonic) + bump(1800.0, harmonic) + bump(2570.0, harmonic),
            duration,
            mixer,
        );
    }

    thread::sleep(Duration::from_millis(duration));
}

fn oo(frequency: f32, duration: u64, mixer: &Mixer) {
    sine(frequency, 1.0, duration, mixer);
    for i in 2..=20 {
        let harmonic = frequency * (i as f32);
        sine(
            harmonic,
            bump(320.0, harmonic) + bump(900.0, harmonic) + bump(2200.0, harmonic),
            duration,
            mixer,
        );
    }

    thread::sleep(Duration::from_millis(duration));
}

fn e(frequency: f32, duration: u64, mixer: &Mixer) {
    sine(frequency, 1.0, duration, mixer);
    for i in 2..=20 {
        let harmonic = frequency * (i as f32);
        sine(
            harmonic,
            bump(330.0, harmonic) + bump(2020.0, harmonic) + bump(2300.0, harmonic),
            duration,
            mixer,
        );
    }

    thread::sleep(Duration::from_millis(duration));
}

fn v(frequency: f32, duration: u64, mixer: &Mixer) {
    sine(frequency, 1.0, duration, mixer);
    for i in 2..=20 {
        let harmonic = frequency * (i as f32);
        sine(
            harmonic,
            bump(220.0, harmonic) + bump(1100.0, harmonic) + bump(2080.0, harmonic),
            duration,
            mixer,
        );
    }

    thread::sleep(Duration::from_millis(duration));
}

fn o(frequency: f32, duration: u64, mixer: &Mixer) {
    sine(frequency, 1.0, duration, mixer);
    for i in 2..=20 {
        let harmonic = frequency * (i as f32);
        sine(
            harmonic,
            bump(540.0, harmonic) + bump(1100.0, harmonic) + bump(2300.0, harmonic),
            duration,
            mixer,
        );
    }

    thread::sleep(Duration::from_millis(duration));
}

fn a(frequency: f32, duration: u64, mixer: &Mixer) {
    sine(frequency, 1.0, duration, mixer);
    for i in 2..=20 {
        let harmonic = frequency * (i as f32);
        sine(
            harmonic,
            bump(620.0, harmonic) + bump(1660.0, harmonic) + bump(2430.0, harmonic),
            duration,
            mixer,
        );
    }

    thread::sleep(Duration::from_millis(duration));
}

fn uh(frequency: f32, duration: u64, mixer: &Mixer) {
    sine(frequency, 1.0, duration, mixer);
    for i in 2..=20 {
        let harmonic = frequency * (i as f32);
        sine(
            harmonic,
            bump(620.0, harmonic) + bump(1200.0, harmonic) + bump(2150.0, harmonic),
            duration,
            mixer,
        );
    }

    thread::sleep(Duration::from_millis(duration));
}

fn r(frequency: f32, duration: u64, mixer: &Mixer) {
    sine(frequency, 1.0, duration, mixer);
    for i in 2..=20 {
        let harmonic = frequency * (i as f32);
        sine(
            harmonic,
            bump(310.0, harmonic) + bump(1060.0, harmonic) + bump(1380.0, harmonic),
            duration,
            mixer,
        );
    }

    thread::sleep(Duration::from_millis(duration));
}

fn m(frequency: f32, duration: u64, mixer: &Mixer) {
    sine(270.0, 0.5, duration, mixer);
    sine(450.0, 0.5, duration, mixer);
    sine(frequency, 1.0, duration, mixer);
    for i in 2..=20 {
        let harmonic = frequency * (i as f32);
        sine(
            harmonic,
            bump(480.0, harmonic) + bump(1270.0, harmonic) + bump(2130.0, harmonic),
            duration,
            mixer,
        );
    }

    thread::sleep(Duration::from_millis(duration));
}

fn d(mixer: &Mixer) {
    sine(200.0, 1.0, 10, mixer);
    sine(1600.0, 0.1, 10, mixer);
    sine(2600.0, 0.5, 10, mixer);

    thread::sleep(Duration::from_millis(10));
}

fn g(mixer: &Mixer) {
    sine(200.0, 1.0, 10, mixer);
    sine(1990.0, 0.1, 10, mixer);
    sine(2850.0, 0.5, 10, mixer);

    thread::sleep(Duration::from_millis(10));
}

fn s(duration: u64, mixer: &Mixer) {
    let sample_rate = 44100;
    let mut data: Vec<f32> = Vec::new();

    for _ in 0..(sample_rate / 1000 * duration) {
        data.push(rand::random_range(-0.3..0.3));
    }

    let buffer = SamplesBuffer::new(
        ChannelCount::new(1).unwrap(),
        SampleRate::new(sample_rate as u32).unwrap(),
        data
    );

    mixer.add(buffer);
    thread::sleep(Duration::from_millis(duration));
}

fn main() {
    let handle = rodio::DeviceSinkBuilder::open_default_sink().expect("open default audio stream");
    let mixer = handle.mixer();

    let octave_mul = 0.75;

    d(mixer);
    e(370.00 * octave_mul, 100, mixer);
    ee(370.00 * octave_mul, 1000, mixer);
    s(50, mixer);
    ee(311.12 * octave_mul, 1000, mixer);

    d(mixer);
    e(246.94 * octave_mul, 100, mixer);
    ee(246.94 * octave_mul, 1000, mixer);
    s(50, mixer);
    ee(185.00 * octave_mul, 1000, mixer);

    g(mixer);
    ih(207.65 * octave_mul, 300, mixer);
    v(207.65 * octave_mul, 100, mixer);

    m(233.08 * octave_mul, 100, mixer);
    ee(233.08 * octave_mul, 300, mixer);

    ee(246.94 * octave_mul, 100, mixer);
    o(246.94 * octave_mul, 200, mixer);
    r(246.94 * octave_mul, 100, mixer);

    a(207.65 * octave_mul, 500, mixer);
    m(207.65 * octave_mul, 100, mixer);
    s(40, mixer);
    uh(246.94 * octave_mul, 400, mixer);
    r(246.94 * octave_mul, 75, mixer);

    d(mixer);
    oo(185.00 * octave_mul, 1000, mixer);

    // ah(250.00, 3000, mixer);
    // ee(250.00, 1000, mixer);
    // oo(250.00, 1000, mixer);
    
    // g(mixer);
    // e(110.0, 100, mixer);
    // ee(110.0, 900, mixer);
}
