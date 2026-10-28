use rodio::{ChannelCount, SampleRate, buffer::SamplesBuffer};

use std::{f32, fs::File, io::Write, thread, time::Duration};

use speech::SAMPLE_RATE;
use speech::consonants::{fricative, nasal, plosive, sonor};
use speech::vowels;

fn main() {
    let handle = rodio::DeviceSinkBuilder::open_default_sink().expect("open default audio stream");
    let mixer = handle.mixer();

    let octave_mul = 0.8;

    let mut wave: Vec<f32> = Vec::new();

    wave.extend(plosive::d());
    wave.extend(vowels::e(370.00 * octave_mul, 900));
    wave.extend(vowels::ee(370.00 * octave_mul, 100));
    wave.extend(fricative::s(50));
    wave.extend(vowels::ee(311.12 * octave_mul, 950));

    wave.extend(plosive::d());
    wave.extend(vowels::e(246.94 * octave_mul, 900));
    wave.extend(vowels::ee(246.94 * octave_mul, 100));
    wave.extend(fricative::s(50));
    wave.extend(vowels::ee(185.00 * octave_mul, 950));

    wave.extend(plosive::g());
    wave.extend(vowels::ih(207.65 * octave_mul, 200));
    wave.extend(fricative::v(207.65 * octave_mul, 100));

    wave.extend(nasal::m(233.08 * octave_mul, 100));
    wave.extend(vowels::ee(233.08 * octave_mul, 200));

    wave.extend(vowels::ee(246.94 * octave_mul, 100));
    wave.extend(vowels::o(246.94 * octave_mul, 200));
    wave.extend(sonor::r(246.94 * octave_mul, 100));

    wave.extend(vowels::a(207.65 * octave_mul, 500));
    wave.extend(nasal::m(207.65 * octave_mul, 100));
    wave.extend(fricative::s(40));
    wave.extend(vowels::uh(246.94 * octave_mul, 300));
    wave.extend(sonor::r(246.94 * octave_mul, 100));

    wave.extend(plosive::d());
    wave.extend(vowels::oo(185.00 * octave_mul, 1000));

    wave.extend(vec![0.0; SAMPLE_RATE as usize]);

    wave.extend(vowels::ah(277.18 * octave_mul, 800));
    wave.extend(vowels::ee(277.18 * octave_mul, 100));
    wave.extend(nasal::m(277.18 * octave_mul, 100));

    wave.extend(vowels::a(370.00 * octave_mul, 900));
    wave.extend(fricative::f(370.00 * octave_mul, 100));

    wave.extend(plosive::k());
    wave.extend(sonor::r(311.12 * octave_mul, 100));
    wave.extend(vowels::e(311.12 * octave_mul, 800));
    wave.extend(vowels::ee(311.12 * octave_mul, 100));
    wave.extend(fricative::s(50));
    wave.extend(vowels::ee(246.94 * octave_mul, 950));

    wave.extend(vowels::ah(207.65 * octave_mul, 100));
    wave.extend(sonor::l(207.65 * octave_mul, 200));

    wave.extend(fricative::f(233.08 * octave_mul, 100));
    wave.extend(vowels::o(233.08 * octave_mul, 100));
    wave.extend(sonor::r(233.08 * octave_mul, 100));

    wave.extend(plosive::d());
    wave.extend(vowels::uh(246.94 * octave_mul, 300));

    wave.extend(sonor::l(277.18 * octave_mul, 100));
    wave.extend(vowels::uh(277.18 * octave_mul, 600));
    wave.extend(fricative::v(277.18 * octave_mul, 100));

    wave.extend(vowels::uh(311.13 * octave_mul, 200));
    wave.extend(fricative::v(311.13 * octave_mul, 100));

    wave.extend(vowels::ee(277.18 * octave_mul, 100));
    wave.extend(vowels::oo(277.18 * octave_mul, 900));

    // wave.extend(vowels::ah(250.00, 3000));
    // wave.extend(vowels::ee(250.00, 1000));
    // wave.extend(vowels::oo(250.00, 1000));

    let mut out = File::create("out.raw").unwrap();
    let _ = out.write_all(unsafe {
        std::slice::from_raw_parts(wave.as_ptr() as *const u8, wave.len() * 4)
    });

    let buffer = SamplesBuffer::new(
        ChannelCount::new(1).unwrap(),
        SampleRate::new(SAMPLE_RATE as u32).unwrap(),
        wave,
    );

    mixer.add(buffer);

    thread::sleep(Duration::from_millis(16000));

    // g();
    // e(110.0, 100);
    // ee(110.0, 900);

    // m(277.18 * octave_mul, 1000);
}
