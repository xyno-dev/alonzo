use crate::sine;

pub fn d() -> Vec<f32> {
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

pub fn g() -> Vec<f32> {
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

pub fn k() -> Vec<f32> {
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
