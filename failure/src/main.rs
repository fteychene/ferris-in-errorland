use std::fs::File;
use std::io::{BufReader, BufRead};
use rand::thread_rng;
use rand::seq::SliceRandom;
use rand::prelude::ThreadRng;

#[macro_use]
extern crate failure;
use failure::{Error, Fail, ResultExt};

#[derive(Debug)]
struct Attendee {
    name: String
}

#[derive(Debug, Fail)]
#[fail(display = "Invalid lottery not enough participant: {} vs {}", asked, existing)]
struct NotEnoughParticipant {
    asked: usize,
    existing: usize
}

fn load_attendees(filename: &str) -> Result<Vec<Attendee>, Error> {
    let file = File::open(filename).context(format!("Error opening file {}", filename))?;
    BufReader::new(file).lines()
        .map(|read| read.map(|value| Attendee { name: value }).map_err(|err| err.into()))
        .collect()
}

fn lottery<'a>(rng: &mut ThreadRng, attendees: &'a Vec<Attendee>, nb: usize) -> Result<Vec<&'a Attendee>, Error> {
    let sample = attendees.choose_multiple(rng,nb)
        .collect::<Vec<_>>();
    if sample.len() != nb { bail!(NotEnoughParticipant{ asked: nb, existing: sample.len()})}
    Ok(sample)
}

fn main() -> Result<(), Error> {
    let mut rng = thread_rng();
    let attendees = load_attendees("attendees.txt")?;
    println!("Winning 3 on attendees.txt : {:?}", lottery(&mut rng, &attendees, 3));
    println!("Error business : {}", lottery(&mut rng, &attendees, 6).unwrap_err());
    println!("Error io : {:?}", load_attendees("attendees2.txt").unwrap_err());
    Ok(())
}
