use crate::logic::parse;
use std::{error, io};
use std::io::Write;

mod logic;
mod music;

type Semitones = f32;
type Melody = Vec<Semitones>;

fn main() -> Result<(), Box<dyn error::Error>> {
    // Play a couple of semitones
    // play(sandstorm())
    // Play the standard scale of music
    // play(scale());

    let mut melody = Melody::new();
    let mut line = String::from("");
    let mut bpm = 120.0;

    println!("Type \"help\" for help!");

    loop {
        print!("-> ");
        io::stdout().flush().expect("Failed to print to stdout!");

        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line!");

        let result = parse(&mut melody, &mut bpm, &line);
        println!("{result}");

        line.clear();
    }
}
