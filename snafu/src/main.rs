use std::fs::File;
use std::io::{BufRead, BufReader};

use rand::prelude::ThreadRng;
use rand::seq::SliceRandom;
use rand::thread_rng;
use snafu::{ensure, ResultExt, Snafu};

#[derive(Debug)]
struct Attendee {
    name: String
}

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("Invalid lottery not enough participant: {} vs {}", asked, existing))]
    NotEnoughParticipant {
        asked: usize,
        existing: usize,
    },
    #[snafu(display("Could not load attendees {}: {}", filename, source))]
    LoadAttendees {
        source: std::io::Error,
        filename: String,
    },
    #[snafu(display("Could not read attendees file : {}", source))]
    ReadAttendeesFile { source: std::io::Error },
}

type Result<T, E = Error> = std::result::Result<T, E>;

fn load_attendees(filename: &str) -> Result<Vec<Attendee>, Error> {
    let file = File::open(filename).context(LoadAttendees { filename: filename.to_string() })?;
    BufReader::new(file).lines()
        .map(|read| read.map(|value| Attendee { name: value }))
        .collect::<Result<Vec<Attendee>, std::io::Error>>()
        .context(ReadAttendeesFile)
}

fn lottery<'a>(rng: &mut ThreadRng, attendees: &'a Vec<Attendee>, nb: usize) -> Result<Vec<&'a Attendee>, Error> {
    let sample = attendees.choose_multiple(rng, nb)
        .collect::<Vec<_>>();
    ensure!(sample.len() == nb, NotEnoughParticipant { asked: nb, existing: sample.len() });
    Ok(sample)
}

fn main() -> Result<(), Error> {
    let mut rng = thread_rng();
    let attendees = load_attendees("attendees.txt")?;
    println!("Winning 3 on attendees.txt : {:?}", lottery(&mut rng, &attendees, 3));
    println!("Error business : {}", lottery(&mut rng, &attendees, 6).unwrap_err());
    println!("Error io : {}", load_attendees("attendees2.txt").unwrap_err());
    Ok(())
}

