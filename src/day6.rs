use crate::day6::Operator::{Addition, Multiplication};
use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};

enum Operator {
    Empty,
    Addition,
    Multiplication,
}

impl Operator {
    fn apply(&self, values: &[u64]) -> u64{
        match self {
            Operator::Empty => {panic!("An Operator should have been found")}
            Addition => {values.iter().sum()}
            Multiplication => {values.iter().product::<u64>()}
        }
    }

}

pub fn execute() -> Result<()> {
    let file = File::open("ressources/day6")?;
    // let file = File::open("ressources/day6example")?;

    let reader = BufReader::new(file);
    let mut values: Vec<Vec<u64>> = Vec::new();
    let mut operators: Vec<String> = Vec::new();

    let mut lines_string: Vec<String> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        lines_string.push(line.clone());
        if line.contains("+") {
            operators.extend(line.split_whitespace().map(|string| string.to_string()));
        } else {
            let splitted_line = line
                .split_whitespace()
                .map(|x| x.trim().parse().expect("numbers should be parsable"))
                .collect();
            values.push(splitted_line);
        }
    }

    let result_part2 = part2(&lines_string);
    let result_part1 = part1(&values, &operators);

    println!("result part 1 = {}", result_part1);
    println!("result part 2 = {}", result_part2);
    Ok(())
}

fn part2(lines: &[String]) -> u64 {
    let cols: Vec<Vec<char>> = lines
        .iter()
        .fold(Vec::<Vec<char>>::new(), |new_vec, string| {
            if new_vec.is_empty() {
                string.chars().map(|x| vec![x]).collect()
            } else {
                new_vec
                    .into_iter()
                    .zip(string.chars())
                    .map(|(mut vec, value)| {
                        vec.push(value);
                        vec
                    })
                    .collect()
            }
        });

    let mut result = 0;
    let mut operator = Operator::Empty;
    let mut values = Vec::<u64>::new();

    for col in cols {
        let mut digits = Vec::<u64>::new();
        let mut empty = true;

        for char in col {
            match char {
                '+' => {
                    operator = Addition;
                    empty = false;
                }
                '*' => {
                    operator = Multiplication;
                    empty = false;
                }
                '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => {
                    digits.push(char.to_digit(10).expect("inputs should only be digits") as u64);
                    empty = false;
                }
                _ => {}
            }
        }

        if empty {
            let problem_result = operator.apply(&values);
            result += problem_result;

            operator = Operator::Empty;
            values = Vec::<u64>::new();
            continue
        }
        if !digits.is_empty() {
                let len = digits.len();

            values.push(digits.iter()
                .enumerate()
                .map(|(digit_index, digit)| -> u64 {
                    *digit * 10_u64.pow((len - 1 - digit_index) as u32)
                })
                .sum());
        }

    }
    let problem_result = operator.apply(&values);
    result += problem_result;
    result
}

fn part1(values: &[Vec<u64>], operators: &[String]) -> u64 {
    //inversion du sens des array, pour passer d'une liste par ligne à une liste par colonne
    let values = values
        .iter()
        .fold(Vec::<Vec<u64>>::new(), |new_vec, old_vec| {
            if new_vec.is_empty() {
                old_vec.iter().map(|x| vec![*x]).collect()
            } else {
                new_vec
                    .into_iter()
                    .zip(old_vec)
                    .map(|(mut vec, value)| {
                        vec.push(*value);
                        vec
                    })
                    .collect()
            }
        });

    let result_part1: u64 = operators
        .iter()
        .zip(values)
        .map(|(operator, values)| match operator.as_str() {
            "+" => values.iter().sum(),
            "*" => values.iter().product::<u64>(),
            _ => panic!("operator should either be * or +"),
        })
        .sum();
    result_part1
}
