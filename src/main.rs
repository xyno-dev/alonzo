use rodio::{
    ChannelCount, SampleRate, Source, buffer::SamplesBuffer, mixer::Mixer, source::SineWave,
};
use std::{
    f32::{self, consts::PI}, fs::{self, File}, io::Write, thread, time::Duration
};

const SAMPLE_RATE: u64 = 40000;

fn sine(frequency: f32, amplitude: f32, duration: u64) -> Vec<f32> {
    let wave: Vec<f32> = (0..(SAMPLE_RATE / 1000 * duration))
        .map(|x| x as f32)
        .map(|x| x * ((2.0 * PI * frequency) / SAMPLE_RATE as f32))
        .map(|x| x.sin())
        .map(|x| x * amplitude)
        .collect();

    wave
}

// Gaussian bump function :D
fn bump(center: f32, x: f32) -> f32 {
    let peak = 1.0;
    let width = 125.0;

    println!(
        "{center}: {}",
        peak * f32::consts::E.powf((-1.0 * (x - center).powi(2)) / (width * width * 2.0))
    );

    peak * f32::consts::E.powf((-1.0 * (x - center).powi(2)) / (width * width * 2.0))
}

fn ah(frequency: f32, duration: u64, mixer: &Mixer) -> Vec<f32> {
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

fn ee(frequency: f32, duration: u64, mixer: &Mixer) -> Vec<f32> {
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

fn ih(frequency: f32, duration: u64, mixer: &Mixer) -> Vec<f32> {
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

fn oo(frequency: f32, duration: u64, mixer: &Mixer) -> Vec<f32> {
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

fn e(frequency: f32, duration: u64, mixer: &Mixer) -> Vec<f32> {
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

fn v(frequency: f32, duration: u64, mixer: &Mixer) -> Vec<f32> {
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

fn f(frequency: f32, duration: u64, mixer: &Mixer) -> Vec<f32> {
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

fn o(frequency: f32, duration: u64, mixer: &Mixer) -> Vec<f32> {
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

fn a(frequency: f32, duration: u64, mixer: &Mixer) -> Vec<f32> {
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

fn uh(frequency: f32, duration: u64, mixer: &Mixer) -> Vec<f32> {
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

fn r(frequency: f32, duration: u64, mixer: &Mixer) -> Vec<f32> {
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

fn l(frequency: f32, duration: u64, mixer: &Mixer) -> Vec<f32> {
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

fn m(frequency: f32, duration: u64, mixer: &Mixer) -> Vec<f32> {
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

fn d(mixer: &Mixer) -> Vec<f32> {
    let mut wave = sine(200.0, 1.0, 10);
    wave = sine(1600.0, 0.1, 10)
        .iter()
        .zip(wave)
        .map(|(x, y)| x + y)
        .collect();
    wave = sine(2600.0, 0.5, 10)
        .iter()
        .zip(wave)
        .map(|(x, y)| x + y)
        .collect();

    wave
}

fn g(mixer: &Mixer) -> Vec<f32> {
    let mut wave = sine(200.0, 1.0, 10);
    wave = sine(1990.0, 0.1, 10)
        .iter()
        .zip(wave)
        .map(|(x, y)| x + y)
        .collect();
    wave = sine(2850.0, 0.5, 10)
        .iter()
        .zip(wave)
        .map(|(x, y)| x + y)
        .collect();

    wave
}

fn k(mixer: &Mixer) -> Vec<f32> {
    let mut wave = sine(300.0, 0.7, 10);
    wave = sine(1700.0, 1.0, 10)
        .iter()
        .zip(wave)
        .map(|(x, y)| x + y)
        .collect();
    wave = sine(2850.0, 0.3, 10)
        .iter()
        .zip(wave)
        .map(|(x, y)| x + y)
        .collect();

    wave
}

fn s(duration: u64, mixer: &Mixer) -> Vec<f32> {
    let mut wave: Vec<f32> = Vec::new();

    for _ in 0..(SAMPLE_RATE / 1000 * duration) {
        wave.push(rand::random_range(-0.3..0.3));
    }

    wave
}

fn main() {
    let handle = rodio::DeviceSinkBuilder::open_default_sink().expect("open default audio stream");
    let mixer = handle.mixer();

    let octave_mul = 0.6;

    let mut wave: Vec<f32> = Vec::new();

    wave.extend(d(mixer));
    wave.extend(e(370.00 * octave_mul, 900, mixer));
    wave.extend(ee(370.00 * octave_mul, 100, mixer));
    wave.extend(s(50, mixer));
    wave.extend(ee(311.12 * octave_mul, 950, mixer));

    wave.extend(d(mixer));
    wave.extend(e(246.94 * octave_mul, 900, mixer));
    wave.extend(ee(246.94 * octave_mul, 100, mixer));
    wave.extend(s(50, mixer));
    wave.extend(ee(185.00 * octave_mul, 950, mixer));

    wave.extend(g(mixer));
    wave.extend(ih(207.65 * octave_mul, 200, mixer));
    wave.extend(v(207.65 * octave_mul, 100, mixer));

    wave.extend(m(233.08 * octave_mul, 100, mixer));
    wave.extend(ee(233.08 * octave_mul, 200, mixer));

    wave.extend(ee(246.94 * octave_mul, 100, mixer));
    wave.extend(o(246.94 * octave_mul, 200, mixer));
    wave.extend(r(246.94 * octave_mul, 100, mixer));

    wave.extend(a(207.65 * octave_mul, 500, mixer));
    wave.extend(m(207.65 * octave_mul, 100, mixer));
    wave.extend(s(40, mixer));
    wave.extend(uh(246.94 * octave_mul, 300, mixer));
    wave.extend(r(246.94 * octave_mul, 100, mixer));

    wave.extend(d(mixer));
    wave.extend(oo(185.00 * octave_mul, 1000, mixer));

    wave.extend(vec![0.0; SAMPLE_RATE as usize]);

    wave.extend(ah(277.18 * octave_mul, 800, mixer));
    wave.extend(ee(277.18 * octave_mul, 100, mixer));
    wave.extend(m(277.18 * octave_mul, 100, mixer));

    wave.extend(a(370.00 * octave_mul, 900, mixer));
    wave.extend(f(370.00 * octave_mul, 100, mixer));

    wave.extend(k(mixer));
    wave.extend(r(311.12 * octave_mul, 100, mixer));
    wave.extend(e(311.12 * octave_mul, 800, mixer));
    wave.extend(ee(311.12 * octave_mul, 100, mixer));
    wave.extend(s(50, mixer));
    wave.extend(ee(246.94 * octave_mul, 950, mixer));

    wave.extend(ah(207.65 * octave_mul, 100, mixer));
    wave.extend(l(207.65 * octave_mul, 200, mixer));

    wave.extend(f(233.08 * octave_mul, 100, mixer));
    wave.extend(o(233.08 * octave_mul, 100, mixer));
    wave.extend(r(233.08 * octave_mul, 100, mixer));

    wave.extend(d(mixer));
    wave.extend(uh(246.94 * octave_mul, 300, mixer));

    wave.extend(l(277.18 * octave_mul, 100, mixer));
    wave.extend(uh(277.18 * octave_mul, 600, mixer));
    wave.extend(v(277.18 * octave_mul, 100, mixer));

    wave.extend(uh(311.13 * octave_mul, 200, mixer));
    wave.extend(v(311.13 * octave_mul, 100, mixer));

    wave.extend(ee(277.18 * octave_mul, 100, mixer));
    wave.extend(oo(277.18 * octave_mul, 900, mixer));
    
    // wave.extend(ah(250.00, 3000, mixer));
    // wave.extend(ee(250.00, 1000, mixer));
    // wave.extend(oo(250.00, 1000, mixer));
    
    let mut out = File::create("out.raw").unwrap();
    let _ = out.write_all(unsafe { std::slice::from_raw_parts(wave.as_ptr() as *const u8, wave.len() * 4) });

    let buffer = SamplesBuffer::new(ChannelCount::new(1).unwrap(), SampleRate::new(SAMPLE_RATE as u32).unwrap(), wave);

    mixer.add(buffer);

    thread::sleep(Duration::from_millis(16000));

    // g(mixer);
    // e(110.0, 100, mixer);
    // ee(110.0, 900, mixer);

    // m(277.18 * octave_mul, 1000, mixer);
}
