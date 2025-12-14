use anyhow::{Error, Result};
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::day8::JunctionError::BadInputs;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum JunctionError {
    #[error("Invalid number of entry: (expected {expected:?}, got {found:?})")]
    BadInputs { expected: usize, found: usize },
}
#[derive(PartialEq, Eq, Hash)]
struct Junction {
    id: usize,
    x: i64,
    y: i64,
    z: i64,
}

impl Junction {
    pub fn from_string(input: &str, id: usize) -> Result<Junction> {
        let splitted: Vec<&str> = input.split(",").collect();

        if splitted.len() != 3 {
            return Err(Error::from(BadInputs {
                expected: 3,
                found: splitted.len(),
            }));
        }

        Ok(Junction {
            id,
            x: splitted
                .first()
                .unwrap()
                .parse()
                .expect("Numbers should be parsable"),
            y: splitted
                .get(1)
                .unwrap()
                .parse()
                .expect("Numbers should be parsable"),
            z: splitted
                .get(2)
                .unwrap()
                .parse()
                .expect("Numbers should be parsable"),
        })
    }

    pub fn distance_squared(&self, other: &Junction) -> i64 {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)
    }
}

#[derive(Debug)]
struct JunctionDistance {
    first: usize,
    second: usize,
    distance: i64,
}
impl JunctionDistance {
    fn new(first: &Junction, second: &Junction) -> JunctionDistance {
        JunctionDistance {
            first: first.id,
            second: second.id,
            distance: first.distance_squared(second),
        }
    }
}
impl PartialEq<Self> for JunctionDistance {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl PartialOrd<Self> for JunctionDistance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for JunctionDistance {}

impl Ord for JunctionDistance {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.cmp(&other.distance)
    }
}

#[derive(Debug)]
struct Networks {
    x: Vec<Vec<usize>>,
}

impl Networks {
    fn add_connection(&mut self, connection: &JunctionDistance) {
        let mut new = vec![connection.first, connection.second];

        let network1 = self.pop_containing(connection.first);
        if let Some(mut network) = network1 {
            new.append(&mut network);
        }

        let network2 = self.pop_containing(connection.second);

        if let Some(mut network) = network2 {
            new.append(&mut network);
        }

        new.sort();
        new.dedup();
        self.x.push(new);
    }
    fn pop_containing(&mut self, junction: usize) -> Option<Vec<usize>> {
        let mut peekable = self
            .x
            .iter()
            .enumerate()
            .filter_map(|(index, network)| {
                if network.contains(&junction) {
                    Some(index)
                } else {
                    None
                }
            })
            .peekable();
        let index = peekable.peek();

        if let Some(index) = index {
            Some(self.x.swap_remove(*index))
        } else {
            None
        }
    }
}

pub fn execute() -> Result<()> {
    let file = File::open("ressources/day8")?;
    // let file = File::open("ressources/day8example")?;

    let reader = BufReader::new(file);

    let junctions: Vec<Junction> = reader
        .lines()
        .enumerate()
        .flat_map(|(id, x)| Junction::from_string(x?.as_str(), id))
        .collect();

    let mut distances: Vec<JunctionDistance> = junctions
        .iter()
        .flat_map(|first| {
            junctions
                .iter()
                .filter_map(|second| {
                    if second.id >= first.id { //déduplication
                        None
                    } else {
                        Some(JunctionDistance::new(first, second))
                    }
                })
                .collect::<Vec<JunctionDistance>>()
        })
        .collect();

    distances.sort_by(|first, second| {first.distance.cmp(&second.distance)});

    println!("input parsed, nb of distances : {}", distances.len());

    let mut networks =  Networks {
        x: Vec::<Vec<usize>>::new()
    };

    let mut index = 0;
    let distances_iter = distances.iter();
    let mut result_part1 = 0;
    let mut result_part2 = 0;
    for distance in distances_iter {
        networks.add_connection(distance);

        index +=1;
        if index == 1000 {
            networks.x.sort_by(|x1, x2| {x2.len().cmp(&x1.len())});
            result_part1= networks.x[0..3].iter().map(|network | { network.len()}).product();
        }

        if networks.x.len() == 1 && networks.x.first().expect("I just checked").len()>3 {
            result_part2 = junctions.iter().filter(|junction| {junction.id == distance.first || junction.id == distance.second}).map(|junction | {junction.x}).product();
            break
        }
    }

    println!("networks calculated, nb of networks : {}, {networks:?}", networks.x.len());

    println!("result part 1 = {}", result_part1);
    println!("result part 2 = {}", result_part2);
    Ok(())
}