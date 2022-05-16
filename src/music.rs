use std::f32::consts::TAU as TWO_PI;
use std::fs::File;
use std::io::Write;
use std::process::Command;
use std::thread;

use crate::logic::PATH;

type Hz = f32;
type Semitones = f32;
type Seconds = f32;
type Sample = f32;
type Beats = f32;
type Melody = Vec<Semitones>;
type Wave = Vec<Sample>;

const SAMPLE_RATE: &str = "48000.0";
const VOLUME: f32 = 0.5;
const PITCH_STANDARD: Hz = 440.0;

// The different pieces of music that are possible to play
pub enum Piece {
    Sandstorm,
    Scale,
    User(Wave),
}

pub fn play(piece: Piece, bpm: Seconds) {
    let mut file = File::create(PATH).unwrap();

    use Piece::*;
    let wave = match piece {
        Sandstorm => sandstorm(bpm),
        Scale => scale(bpm),
        User(melody) => melody,
    };

    for sample in wave {
        // Saved as little-endian, to be played by ffplay
        file.write_all(&sample.to_le_bytes()).unwrap();
    }

    // let args = ["-f", "f32le", "-ar", &sample_rate, "-showmode", "1", path];
    let args = [
        "-f",
        "f32le",
        "-ar",
        SAMPLE_RATE,
        PATH,
        "-nodisp",
        "-autoexit",
    ];
    thread::spawn(move || {
        Command::new("ffplay").args(args).output().unwrap();
    });
}

fn sandstorm(bpm: Seconds) -> Melody {
    let mut melody: Melody = Vec::new();

    melody.extend(note(0.0, bpm, 0.25));
    melody.extend(note(0.0, bpm, 0.25));
    melody.extend(note(0.0, bpm, 0.25));
    melody.extend(note(0.0, bpm, 0.25));
    melody.extend(note(0.0, bpm, 0.5));
    melody.extend(note(0.0, bpm, 0.25));
    melody.extend(note(0.0, bpm, 0.25));
    melody.extend(note(0.0, bpm, 0.25));
    melody.extend(note(0.0, bpm, 0.25));
    melody.extend(note(0.0, bpm, 0.25));
    melody.extend(note(0.0, bpm, 0.25));
    melody.extend(note(0.0, bpm, 0.5));
    melody.extend(note(5.0, bpm, 0.25));
    melody.extend(note(5.0, bpm, 0.25));
    melody.extend(note(5.0, bpm, 0.25));
    melody.extend(note(5.0, bpm, 0.25));
    melody.extend(note(5.0, bpm, 0.25));
    melody.extend(note(5.0, bpm, 0.25));
    melody.extend(note(5.0, bpm, 0.5));
    melody.extend(note(3.0, bpm, 0.25));
    melody.extend(note(3.0, bpm, 0.25));
    melody.extend(note(3.0, bpm, 0.25));
    melody.extend(note(3.0, bpm, 0.25));
    melody.extend(note(3.0, bpm, 0.25));
    melody.extend(note(3.0, bpm, 0.25));
    melody.extend(note(3.0, bpm, 0.5));
    melody.extend(note(-2.0, bpm, 0.5));
    melody.extend(note(0.0, bpm, 0.25));
    melody.extend(note(0.0, bpm, 0.25));
    melody.extend(note(0.0, bpm, 0.25));
    melody.extend(note(0.0, bpm, 0.25));
    melody.extend(note(0.0, bpm, 0.5));

    melody
}

fn f(n: Semitones) -> Hz {
    let a = 2.0_f32.powf(1.0 / 12.0);
    PITCH_STANDARD * a.powf(n)
}

pub fn note(n: Semitones, bpm: Seconds, beats: Beats) -> Wave {
    freq(f(n), beats * (60.0 / bpm))
}

fn freq(frequency: Hz, duration: Seconds) -> Wave {
    let sample_rate = SAMPLE_RATE.parse::<f32>().unwrap();
    let step = (frequency * TWO_PI) / sample_rate;
    let samples = (sample_rate * duration) as i32;

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

fn scale(bpm: Seconds) -> Melody {
    let mut wave: Melody = Vec::new();
    wave.extend(note(0.0, bpm, 1.0));
    wave.extend(note(2.0, bpm, 1.0));
    wave.extend(note(4.0, bpm, 1.0));
    wave.extend(note(5.0, bpm, 1.0));
    wave.extend(note(7.0, bpm, 1.0));
    wave.extend(note(9.0, bpm, 1.0));
    wave.extend(note(11.0, bpm, 1.0));
    wave.extend(note(12.0, bpm, 1.0));
    wave
}
