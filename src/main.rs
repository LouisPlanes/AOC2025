use std::io;
use anyhow::Result;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;


fn main() -> Result<()> {
        println!("choose a day:");
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("error while reading choice");

        let choice :u32 = choice.trim().parse()?;

        match choice {
            1 => Ok(day1::execute()?),
            2 => Ok(day2::execute()?),
            3 => Ok(day3::execute()?),
            4 => Ok(day4::execute()?),
            5 => Ok(day5::execute()?),
            6 => Ok(day6::execute()?),
            7 => Ok(day7::execute()?),
            _ => {
                println!("Unknown day!");
                Ok(())
            },
    }
}
