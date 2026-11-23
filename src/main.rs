use rodio::{ChannelCount, SampleRate, buffer::SamplesBuffer};

use std::{
    env::{ArgsOs, args_os},
    f32,
    fs::File,
    io::{Read, Write},
    thread,
    time::Duration,
};

use speech::SAMPLE_RATE;
use speech::consonants::{fricative, nasal, plosive, sonor};
use speech::vowels;
use speech::wav::Wav;

fn print_usage_text(args: &mut ArgsOs) {
    let path = match args.nth(0) {
        None => String::from("alonzo"),
        Some(path) => path.into_string().unwrap(),
    };

    println!("Usage: {path} FILE");
}

fn main() {
    let mut handle =
        rodio::DeviceSinkBuilder::open_default_sink().expect("open default audio stream");
    handle.log_on_drop(false);
    let mixer = handle.mixer();

    let sonor_generators = sonor::SonorGeneratorCollection::new();
    let fricative_generators = fricative::FricativeGeneratorCollection::new();
    let plosive_generators = plosive::PlosiveGeneratorCollection::new();
    let nasal_generators = nasal::NasalGeneratorCollection::new();
    let vowel_generators = vowels::VowelGeneratorCollection::new();

    let mut wave: Vec<f32> = Vec::new();

    let mut args = args_os();

    let mut file = match args.nth(1) {
        Some(filename) => File::open(filename).expect("Failed to open phoneme file"),
        None => {
            print_usage_text(&mut args);
            return;
        }
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read phoneme file");

    let tokens = contents.split_whitespace();

    let mut frequency: f32 = 0.0;
    let mut duration: u64 = 0;

    for token in tokens {
        if token.len() > 2 {
            match token.chars().nth(0).unwrap() {
                'f' => frequency = token[1..].parse::<f32>().expect("Frequency must be f32!"),
                'd' => duration = token[1..].parse::<u64>().expect("Duration must be u64!"),
                'p' => {
                    let duration = token[1..]
                        .parse::<u64>()
                        .expect("Pause duration must be u64!");
                    wave.extend(vec![0.01; (SAMPLE_RATE / 1000 * duration) as usize]);
                }
                _ => {}
            }
        } else {
            wave.extend(match token {
                "r" => sonor_generators.r.generate(frequency, duration),
                "l" => sonor_generators.l.generate(frequency, duration),

                "f" => fricative_generators.f.generate(frequency, duration),
                "v" => fricative_generators.v.generate(frequency, duration),
                "s" => fricative_generators.s.generate(duration),

                "d" => plosive_generators.d.generate(duration),
                "g" => plosive_generators.g.generate(duration),
                "k" => plosive_generators.k.generate(duration),
                "t" => plosive_generators.t.generate(duration),

                "m" => nasal_generators.m.generate(frequency, duration),

                "ah" => vowel_generators.ah.generate(frequency, duration),
                "ee" => vowel_generators.ee.generate(frequency, duration),
                "ih" => vowel_generators.ih.generate(frequency, duration),
                "oo" => vowel_generators.oo.generate(frequency, duration),
                "uh" => vowel_generators.uh.generate(frequency, duration),
                "a" => vowel_generators.a.generate(frequency, duration),
                "e" => vowel_generators.e.generate(frequency, duration),
                "o" => vowel_generators.o.generate(frequency, duration),

                _ => Vec::new(),
            });
        }
    }

    // wave.extend(vowels::ah(250.00, 3000));
    // wave.extend(vowels::ee(250.00, 1000));
    // wave.extend(vowels::oo(250.00, 1000));

    let total_duration = wave.len() as f32 / SAMPLE_RATE as f32;

    let mut out = File::create("out.wav").unwrap();
    let wav_file = Wav::from(&wave);
    let _ = out.write_all(&wav_file.generate());

    let buffer = SamplesBuffer::new(
        ChannelCount::new(1).unwrap(),
        SampleRate::new(SAMPLE_RATE as u32).unwrap(),
        wave,
    );

    mixer.add(buffer);

    thread::sleep(Duration::from_secs_f32(total_duration + 0.1));
}
