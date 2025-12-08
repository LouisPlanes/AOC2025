use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::ops::Range;

pub fn part1() -> Result<(), Error> {
    let file = File::open("ressources/day2")?;

    let reader = BufReader::new(file);

    let mut ranges: Vec<Range<i64>> = vec![];

    for line in reader.lines() {
        let string = line?;
        let split = string.split(",");
        for range_str in split {
            let splited = range_str
                .split_once("-")
                .expect("ranges should be splittable");
            let left = splited.0.trim().parse().expect("number should be parsable");
            let right: i64 = splited.1.trim().parse().expect("number should be parsable");

            ranges.push(left..right + 1)
        }
    }

    let mut count_part1: i64 = 0;
    let mut count_part2: i64 = 0;

    for range in ranges {
        for number in range {
            let stringified = number.to_string();

            let len = stringified.len();
            for parts_number in 2..len+1 {
                if (len % parts_number) != 0 {
                    continue;
                }

                let mut splitted = vec![];
                let stringified_clone = stringified.clone();
                let mut cur = stringified_clone.as_str();
                while !cur.is_empty() {
                    let (chunk, rest) = cur.split_at(len / parts_number);
                    splitted.push(chunk);
                    cur = rest;
                }

                let mut iter = splitted.into_iter();

                let value = iter.next();

                let all_equals = iter
                    .fold(value, |acc, other| {
                        acc.and_then(|stored| if stored == other { Some(stored) } else { None })
                    })
                    .is_some();

                if all_equals {

                    println!("number matched = {}", number);
                    count_part2 += number;
                    break
                }
            }

            if (len % 2) != 0 {
                continue;
            }
            let splitted = stringified.split_at(len / 2);
            if splitted.1 == splitted.0 {
                count_part1 += number;
            }
        }
    }
    println!("result part 1 = {}", count_part1);
    println!("result part 2 = {}", count_part2);
    Ok(())
}
