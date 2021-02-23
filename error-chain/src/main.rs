use std::fs::File;
use std::io::{BufReader, BufRead};
use rand::thread_rng;
use rand::seq::SliceRandom;
use rand::prelude::ThreadRng;

#[macro_use]
extern crate error_chain;
use error_chain::bail;

mod errors {
    error_chain! {
        types {
            Error, ErrorKind, ResultExt, Result;
        }

        foreign_links {
            Io(::std::io::Error);
        }

        // Define additional `ErrorKind` variants.  Define custom responses with the
        // `description` and `display` calls.
        errors {
            NotEnoughParticipant(existing: usize, asked: usize) {
                description("Invalid lottery")
                display("Invalid lottery not enough participant: {} vs {}", asked, existing)
            }
        }
    }
}

use errors::*;

#[derive(Debug)]
struct Attendee {
    name: String
}

fn load_attendees(filename: &str) -> Result<Vec<Attendee>> {
    let file = File::open(filename)?;
    BufReader::new(file).lines()
        .map(|read| read.map(|value| Attendee { name: value }))
        .collect::<std::result::Result<Vec<_>, std::io::Error>>()
        .map_err(|err| err.into())
}

fn lottery<'a>(rng: &mut ThreadRng, attendees: &'a Vec<Attendee>, nb: usize) -> Result<Vec<&'a Attendee>> {
    let sample = attendees.choose_multiple(rng, nb)
        .collect::<Vec<_>>();
    if sample.len() != nb { bail!(ErrorKind::NotEnoughParticipant(sample.len(), nb)) }
    Ok(sample)
}

fn main()-> Result<()> {
    let mut rng = thread_rng();
    let attendees = load_attendees("attendees.txt")?;
    println!("Winning 3 on attendees.txt : {:?}", lottery(&mut rng, &attendees, 3));
    println!("Error business : {}", lottery(&mut rng, &attendees, 6).unwrap_err());
    println!("Error io : {}", load_attendees("attendees2.txt").unwrap_err());
    Ok(())
}
