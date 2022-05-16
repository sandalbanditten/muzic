use std::f32::consts::TAU as TWO_PI;
use std::fs::File;
use std::io::Write;
use std::process::Command;

type Hz = f32;
type Semitones = f32;
type Seconds = f32;
type Sample = f32;
type Beats = f32;
type Melody = Vec<Semitones>;
type Wave = Vec<Sample>;

const SAMPLE_RATE: f32 = 48_000.0;
const VOLUME: f32 = 0.5;
const PITCH_STANDARD: Hz = 440.0;
const BPM: Beats = 120.0;
const BEAT_DURATION: Seconds = 60.0 / BPM;

fn main() {
    // Play a couple of semitones
    play(sandstorm())
    // Play the standard scale of music
    // play(scale());
}

fn play(wave: Wave) {
    let path = "music.bin";
    let mut file = File::create(path).unwrap();
    for sample in wave {
        // Saved as little-endian, to be played by ffplay
        file.write_all(&sample.to_le_bytes()).unwrap();
    }
    let sample_rate = SAMPLE_RATE.to_string();
    // let args = ["-f", "f32le", "-ar", &sample_rate, "-showmode", "1", path];
    let args = [
        "-f",
        "f32le",
        "-ar",
        &sample_rate,
        path,
        "-nodisp",
        "-autoexit",
    ];
    let _output = Command::new("ffplay").args(args).output().unwrap();
}

fn sandstorm() -> Melody {
    let mut melody: Melody = Vec::new();

    melody.extend(note(0.0, 0.25));
    melody.extend(note(0.0, 0.25));
    melody.extend(note(0.0, 0.25));
    melody.extend(note(0.0, 0.25));
    melody.extend(note(0.0, 0.5));
    melody.extend(note(0.0, 0.25));
    melody.extend(note(0.0, 0.25));
    melody.extend(note(0.0, 0.25));
    melody.extend(note(0.0, 0.25));
    melody.extend(note(0.0, 0.25));
    melody.extend(note(0.0, 0.25));
    melody.extend(note(0.0, 0.5));

    melody.extend(note(5.0, 0.25));
    melody.extend(note(5.0, 0.25));
    melody.extend(note(5.0, 0.25));
    melody.extend(note(5.0, 0.25));
    melody.extend(note(5.0, 0.25));
    melody.extend(note(5.0, 0.25));
    melody.extend(note(5.0, 0.5));

    melody.extend(note(3.0, 0.25));
    melody.extend(note(3.0, 0.25));
    melody.extend(note(3.0, 0.25));
    melody.extend(note(3.0, 0.25));
    melody.extend(note(3.0, 0.25));
    melody.extend(note(3.0, 0.25));
    melody.extend(note(3.0, 0.5));

    melody.extend(note(-2.0, 0.5));

    melody.extend(note(0.0, 0.25));
    melody.extend(note(0.0, 0.25));
    melody.extend(note(0.0, 0.25));
    melody.extend(note(0.0, 0.25));
    melody.extend(note(0.0, 0.5));

    melody
}

fn f(n: Semitones) -> Hz {
    let a = 2.0_f32.powf(1.0 / 12.0);
    PITCH_STANDARD * a.powf(n)
}

fn note(n: Semitones, beats: Beats) -> Wave {
    freq(f(n), beats * BEAT_DURATION)
}

fn freq(frequency: Hz, duration: Seconds) -> Wave {
    let step = (frequency * TWO_PI) / SAMPLE_RATE;
    let samples = (SAMPLE_RATE * duration) as i32;

    // Create the basic wave
    let mut wave = Vec::new();
    for i in 0..samples {
        wave.push(i as Sample);
    }

    // Create the attack vec
    let mut attack = Vec::new();
    for i in 0..samples {
        attack.push(1.0_f32.min(i as f32 * 0.001));
    }

    // Create the release vec
    let mut release = attack.clone();
    release.reverse();

    // Create the basic sine wave
    let mut output = wave
        .iter_mut()
        .map(|sample| *sample * step)
        .map(|sample| sample.sin())
        .map(|sample| sample * VOLUME)
        .collect::<Wave>();

    for i in 0..samples {
        // Apply attack and release
        output[i as usize] *= attack[i as usize];
        output[i as usize] *= release[i as usize];
    }

    output
}

fn scale() -> Melody {
    let mut wave: Melody = Vec::new();
    wave.extend(note(0.0, 1.0));
    wave.extend(note(2.0, 1.0));
    wave.extend(note(4.0, 1.0));
    wave.extend(note(5.0, 1.0));
    wave.extend(note(7.0, 1.0));
    wave.extend(note(9.0, 1.0));
    wave.extend(note(11.0, 1.0));
    wave.extend(note(12.0, 1.0));
    wave
}
