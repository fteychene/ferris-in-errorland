use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

use rand::prelude::ThreadRng;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug)]
struct Attendee {
    name: String
}

#[derive(Debug)]
struct NotEnoughParticipant {
    asked: usize,
    existing: usize,
}

impl fmt::Display for NotEnoughParticipant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid lottery not enough participant: {} vs {}", self.asked, self.existing)
    }
}

impl Error for NotEnoughParticipant {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

fn load_attendees(filename: &str) -> Result<Vec<Attendee>, Box<dyn Error>> {
    let file = File::open(filename)?;
    BufReader::new(file).lines()
        .map(|read| read.map(|value| Attendee { name: value }))
        .collect::<Result<Vec<Attendee>, std::io::Error>>()
        .map_err(|err| err.into())
}

fn lottery<'a>(rng: &mut ThreadRng, attendees: &'a Vec<Attendee>, nb: usize) -> Result<Vec<&'a Attendee>, Box<dyn Error>> {
    let sample = attendees.choose_multiple(rng, nb)
        .collect::<Vec<_>>();
    if sample.len() != nb { return Err(Box::new(NotEnoughParticipant{ asked: nb, existing: sample.len()})) }
    Ok(sample)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut rng = thread_rng();
    let attendees = load_attendees("attendees.txt")?;
    println!("Winning 3 on attendees.txt : {:?}", lottery(&mut rng, &attendees, 3));
    println!("Error business : {}", lottery(&mut rng, &attendees, 6).unwrap_err());
    println!("Error io : {:?}", load_attendees("attendees2.txt").unwrap_err());
    Ok(())
}
