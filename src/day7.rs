use anyhow::Result;
use std::cmp::{max, min};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Range;

pub fn execute() -> Result<()> {
    let file = File::open("ressources/day7")?;
   // let file = File::open("ressources/day7example")?;

    let mut reader = BufReader::new(file);
    let mut line1 = String::new();
    reader.read_line(&mut line1)?;

    let mut laser: Vec<u64> = vec![0; line1.len()];
    laser.insert((laser.len() / 2) -1, 1);
    let mut result_part1 = 0;

    for line in reader.lines() {
        let mut new_laser: Vec<u64> = vec![0; laser.len()];
        let line = line?;
        line.chars()
            .enumerate()
            .for_each(|(index, char)| match char {
                '^' => {
                    let beam = laser.get(index).expect("should be present");
                    if *beam > 0 {
                        *new_laser.get_mut(index - 1).expect("should be present") += beam;
                        *new_laser.get_mut(index + 1).expect("should be present") += beam;
                        result_part1 += 1;
                    }
                }
                '.' => {
                    let beam = laser.get(index).expect("should be present");
                    *new_laser.get_mut(index).expect("should be present") += beam;
                }
                _ => panic!("unkown char {char}"),
            });
        laser = new_laser;
    }
let result_part2 :u64= laser.iter().sum();
    println!("result part 1 = {}", result_part1);
    println!("result part 2 = {}", result_part2);
    Ok(())
}
