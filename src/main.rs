use rodio::{ChannelCount, SampleRate, buffer::SamplesBuffer};

use std::{
    f32,
    fs::File,
    io::{Read, Write},
    thread,
    time::Duration,
};

use speech::SAMPLE_RATE;
use speech::consonants::{fricative, nasal, plosive, sonor};
use speech::vowels;

fn main() {
    let handle = rodio::DeviceSinkBuilder::open_default_sink().expect("open default audio stream");
    let mixer = handle.mixer();

    let octave_mul = 0.8;

    let mut wave: Vec<f32> = Vec::new();

    let mut file = File::open("daisy.alo").expect("Failed to open phoneme file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read phoneme file");

    let tokens = contents.split_whitespace();

    let mut frequency: f32 = 0.0;
    let mut duration: u64 = 0;

    let mut total_duration: u64 = 0;

    for token in tokens {
        if token.len() > 2 {
            match token.chars().nth(0).unwrap() {
                'f' => frequency = token[1..].parse::<f32>().expect("Frequency must be f32!"),
                'd' => duration = token[1..].parse::<u64>().expect("Duration must be u64!"),
                'p' => wave.extend(vec![
                    0.0;
                    (SAMPLE_RATE / 1000
                        * token[1..]
                            .parse::<u64>()
                            .expect("Pause duration must be u64!"))
                        as usize
                ]),
                _ => {}
            }
        } else {
            match token {
                "r" => wave.extend(sonor::r(frequency, duration)),
                "l" => wave.extend(sonor::l(frequency, duration)),

                "f" => wave.extend(fricative::f(frequency, duration)),
                "v" => wave.extend(fricative::v(frequency, duration)),
                "s" => wave.extend(fricative::s(duration)),

                "d" => wave.extend(plosive::d()),
                "g" => wave.extend(plosive::g()),
                "k" => wave.extend(plosive::k()),

                "m" => wave.extend(nasal::m(frequency, duration)),

                "ah" => wave.extend(vowels::ah(frequency, duration)),
                "ee" => wave.extend(vowels::ee(frequency, duration)),
                "ih" => wave.extend(vowels::ih(frequency, duration)),
                "oo" => wave.extend(vowels::oo(frequency, duration)),
                "uh" => wave.extend(vowels::uh(frequency, duration)),
                "a" => wave.extend(vowels::a(frequency, duration)),
                "e" => wave.extend(vowels::e(frequency, duration)),
                "o" => wave.extend(vowels::o(frequency, duration)),

                _ => {}
            }
            total_duration += duration;
        }
    }

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

    thread::sleep(Duration::from_millis(total_duration));
}
