use rand::prelude::ThreadRng;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Attendee {
    name: String,
}

// Function that will wail with technical errors
fn load_attendees(filename: &str) -> Vec<Attendee> {
    let file = File::open(filename).expect("Something went wrong opening the file");
    BufReader::new(file)
        .lines()
        .map(|read| Attendee {
            name: read.expect("Something went wrong reading line"),
        })
        .collect()
}

// Function that can result with a business error
fn lottery<'a>(rng: &mut ThreadRng, attendees: &'a Vec<Attendee>, nb: usize) -> Vec<&'a Attendee> {
    let sample = attendees.choose_multiple(rng, nb).collect::<Vec<_>>();
    if sample.len() != nb {
        panic!("Error on lottery not enough attendees")
    }
    sample
}

fn main() {
    let mut rng = thread_rng();
    let attendees = load_attendees("attendees.txt");
    println!(
        "Winning 3 on attendees.txt : {:?}",
        lottery(&mut rng, &attendees, 3)
    );
    println!("Error business : {:?}", lottery(&mut rng, &attendees, 6));
    println!("Error io : {:?}", load_attendees("attendees2.txt"));
}
