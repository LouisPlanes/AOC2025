use std::fs::File;
use std::io::{BufRead, BufReader, Error};

pub fn part1() -> Result<(), Error> {
    let file = File::open("ressources/day3")?;

    let reader = BufReader::new(file);

    let mut acc_part1 = 0;
    let len: usize = 12;

    for line in reader.lines() {
        let string = line?;

        let mut digits: Vec<u64> = vec![0; len];

        let last_index = string.len() - 1;

        for (index, char) in string.chars().enumerate() {
            let curr_value: u64 = char.to_digit(10).expect("inputs should only be digits") as u64;

            let mut reseted = false;
            for (digit_index, digit) in digits.iter_mut().enumerate() {
                if reseted {
                    *digit = 0;
                } else if curr_value > *digit {
                    if index <= (last_index - (len - 1 - digit_index)){

                        *digit = curr_value;
                        reseted = true;
                    }
                }
            }
        }
        let joltage: u64 = digits
            .iter()
            .enumerate()
            .map(|(digit_index, digit)| -> u64 {
                *digit * 10_u64.pow((len - 1 - digit_index) as u32)
            })
            .sum();
        println!("line joltage = {}", joltage);

        acc_part1 += joltage;
    }

    println!("result part 1 = {}", acc_part1);
    // println!("result part 2 = {}", count_part2);
    Ok(())
}
