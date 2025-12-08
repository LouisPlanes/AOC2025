use std::fs::File;
use std::io::{BufRead, BufReader, Error};

pub fn part1() -> Result<(), Error> {


    let file = File::open("ressources/day1part1")?;

    let reader = BufReader::new(file);
    let mut dial: i32 = 50;
    let mut result_part1 = 0;
    let mut result_part2 = 0;


    for line in reader.lines() {
        match line {
            Ok(line) => {
                let splited = line.split_at_checked(1);
                let splited = splited.unwrap();

                let delta: i32 = splited.1.trim().parse().unwrap();

                let added = match splited.0 {
                    "L" => {
                        if dial == 0 {
                            result_part2 -= 1
                        }
                        -delta
                    },
                    "R" => delta,
                    other => {
                        panic!("{other}")
                    }
                };

                dial = dial + added;
                while dial < 0 {
                    dial += 100;
                    result_part2 += 1
                }

                while dial >= 100 {
                    dial -= 100;
                    if !(dial == 0) {
                        result_part2 += 1
                    }
                }
                if dial == 0 {
                    result_part1 += 1;
                    result_part2 += 1
                }
                println!("step : {line}, dial : {dial} result temp : {result_part2}");

            }
            Err(err) => {
                println!("error : {}", err);
                break;
            }
        }
    }
    println!("result part 1 :{}", result_part1);
    println!("result part 1 :{}", result_part2);

    Ok(())
}
