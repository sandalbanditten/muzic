use crate::music::{note, play, Piece};

type Semitones = f32;
type Seconds = f32;
type Melody = Vec<Semitones>;

pub const PATH: &str = "/tmp/music.bin";

pub fn parse(melody: &mut Melody, bpm: &mut Seconds, string: &str) -> String {
    let trimmed = string.trim();
    match trimmed.to_lowercase().split(' ').next().unwrap() {
        "sandstorm" => {
            play(Piece::Sandstorm, *bpm);
            String::from("Playing sandstorm")
        }
        "scale" => {
            play(Piece::Scale, *bpm);
            String::from("Playing the musical scale")
        }
        "help" | "h" => print_help(),
        "play" | "p" => {
            if melody.is_empty() {
                String::from("No music yet")
            } else {
                play(Piece::User(melody.clone()), *bpm);
                String::from("Playing your music")
            }
        }
        "quit" | "q" => {
            quit();
            // The code below is unreachable
            String::from("")
        }
        "clear" => {
            melody.clear();
            String::from("Resetting music")
        }
        "add" => parse_add(melody, bpm, string),
        "bpm" => parse_bpm(bpm, string),
        _ => String::from("Invalid input"),
    }
}

fn parse_bpm(bpm: &mut Seconds, string: &str) -> String {
    let result = string.split_whitespace().nth(1);
    if result.is_some() {
        *bpm = result.unwrap().parse::<f32>().unwrap_or(120.0);

        format!("BPM changed to {bpm}")
    } else {
        String::from("Invalid input")
    }
}

fn parse_add(melody: &mut Melody, bpm: &Seconds, string: &str) -> String {
    let mut result = string.split_whitespace();
    let len = result.clone().count();
    if len == 3 {
        result.next();
        let semitones = result
            .next()
            .unwrap_or("0.0")
            .parse::<f32>()
            .unwrap_or(0.0)
            .round();
        let beats = result.next().unwrap_or("0.0").parse::<f32>().unwrap_or(0.0);
        melody.extend(note(semitones, *bpm, beats));
        String::from("Added note")
    } else {
        String::from("Invalid input tone and duration")
    }
}

fn print_help() -> String {
    String::from(
        "\
Some predifined music:
    \"sandstorm\"
    \"scale\"

Play your music:
    play

Reset music:
    clear

Quit:
    quit

Save:
    save PATH

Add notes:
    add SEMITONES BEATS

Set bpm:
    bpm BPM
",
    )
}

fn quit() {
    std::fs::remove_file(PATH).unwrap_or(());
    std::process::exit(0);
}
