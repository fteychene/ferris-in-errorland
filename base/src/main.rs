use std::fs::File;
use std::io::{BufReader, BufRead};
use rand::thread_rng;
use rand::seq::SliceRandom;
use rand::prelude::ThreadRng;

#[derive(Debug)]
struct Attendee {
    name: String
}

fn load_attendees(filename: &str) -> Vec<Attendee> {
    let file = File::open(filename).expect("Something went wrong opening the file");
    BufReader::new(file).lines()
        .map(|read| Attendee { name: read.expect("Something went wrong reading line")})
        .collect()
}

fn lottery<'a>(rng: &mut ThreadRng, attendees: &'a Vec<Attendee>, nb: usize) -> Vec<&'a Attendee> {
    let sample = attendees.choose_multiple(rng,nb)
        .collect::<Vec<_>>();
    if sample.len() != nb { panic!("Error on lottery not enough attendees")}
    sample
}

fn main() {
    let mut rng = thread_rng();
    let attendees = load_attendees("attendees.txt");
    println!("Winning 3 on attendees.txt {:?}", lottery(&mut rng, &attendees, 3));
    println!("Winning 6 on attendees2.txt{:?}", lottery(&mut rng, &attendees, 6));

    let attendees = load_attendees("attendees2.txt");
    println!("Winning 3 on attendees2.txt {:?}", lottery(&mut rng, &attendees, 3));
}
