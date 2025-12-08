use std::io;
use anyhow::Result;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;


fn main() -> Result<()> {
        println!("choose a day, 0 to quit:");
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("error while reading choice");

        let choice :u32 = choice.trim().parse()?;

        match choice {
            1 => Ok(day1::part1()?),
            2 => Ok(day2::part1()?),
            3 => Ok(day3::part1()?),
            4 => Ok(day4::part1()?),
            5 => Ok(day5::part1()?),
            _ => {
                println!("Unknown day!");
                Ok(())
            },
    }
}
