use anyhow::Result;
use std::cmp::{max, min};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Range;

pub fn part1() -> Result<()> {
    let file = File::open("ressources/day5")?;
    // let file = File::open("ressources/day5example")?;

    let reader = BufReader::new(file);
    let mut ranges: Vec<Range<u64>> = Vec::new();
    let mut ingredients: Vec<u64> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if line.contains("-") {
            let splitted = line
                .split_once("-")
                .expect("should be splittable as there is a - in it");
            let first: u64 = splitted.0.trim().parse()?;
            let second: u64 = splitted.1.trim().parse()?;
            ranges.push(first..second + 1)
        } else if !line.is_empty() {
            ingredients.push(line.trim().parse()?)
        }
    }

    let result_part1 = ingredients
        .iter()
        .map(|ingredient| {
            match ranges
                .iter()
                .filter(|range| range.contains(&ingredient))
                .peekable()
                .peek()
            {
                Some(_) => 1,
                None => 0,
            }
        })
        .sum::<i32>();

    // nope nope nope nope
    // let result_part2 = ranges_merged
    //     .into_iter()
    //     .flat_map(|range| {
    //         range.enumerate()
    //     })
    //     .fold(HashMap::<u64, _>::new(), |mut map, (_, value)| {
    //         map.insert(value, ());
    //         map
    //     })
    //     .len();


    #[allow(unused_assignments)]
    let mut ranges_merged: Vec<Range<u64>> = Vec::new();
    let mut old_ranges = ranges;
    loop {
        let old_len = old_ranges.len();
        ranges_merged = Vec::new();
        println!("ranges len = {}", old_len);

        for new_range in old_ranges {
            let mut found = false;
            ranges_merged.iter_mut().for_each(|range_merged| {
                if new_range.start <= range_merged.start && new_range.end >= range_merged.end {
                    *range_merged = new_range.clone();
                    found = true;
                    return;
                }
                if new_range.start >= range_merged.start && new_range.end <= range_merged.end {
                    found = true;
                    return;
                }
                if range_merged.contains(&new_range.start)
                    || range_merged.contains(&(new_range.end))
                {
                    *range_merged = min(new_range.start, range_merged.start)
                        ..max(new_range.end, range_merged.end);
                    found = true;
                }
            });
            if !found {
                ranges_merged.push(new_range);
            }
        }
        if old_len == ranges_merged.len() {
            break;
        }
        old_ranges = ranges_merged.clone()
    }

    let result_part2: u64 = ranges_merged
        .iter()
        .map(|range| range.end - range.start)
        .sum();



    println!("result part 1 = {}", result_part1);
    println!("result part 2 = {}", result_part2);
    Ok(())
}
