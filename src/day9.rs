use crate::day9::Segment::Vertical;
use Segment::Horizontal;
use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};

enum Segment {
    Vertical { x: f64, start_y: f64, end_y: f64 },
    Horizontal { y: f64, start_x: f64, end_x: f64 },
}

impl Segment {
    pub fn cross(&self, other: &Segment) -> bool {
        match (self, other) {
            (Vertical { x, start_y, end_y }, Horizontal { y, start_x, end_x }) => {
                x < start_x && x > end_x && y < start_y && y > end_y
            }
            (Horizontal { y, start_x, end_x }, Vertical { x, start_y, end_y }) => {
                x < start_x && x > end_x && y < start_y && y > end_y
            }
            _ => false,
        }
    }

    pub fn new(start: (f64, f64), end: (f64, f64)) -> Self {
        if start.0 == end.0 {
            return Vertical {
                x: start.0,
                start_y: start.1.max(end.1),
                end_y: start.1.min(end.1),
            };
        }

        if start.1 == end.1 {
            return Horizontal {
                y: start.1,
                start_x: start.0.max(end.0),
                end_x: start.0.min(end.0),
            };
        }
        panic!()
    }
}

pub fn execute() -> Result<()> {
    let file = File::open("ressources/day9")?;
    // let file = File::open("ressources/day9example")?;

    let reader = BufReader::new(file);
    let mut corners: Vec<(f64, f64)> = Vec::new();
    let mut segments: Vec<Segment> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let splitted = line.split_once(",").expect("should be splittable by , ");

        let first: f64 = splitted.0.trim().parse()?;
        let second: f64 = splitted.1.trim().parse()?;
        let current_corner = (first, second);

        let last_corner = corners.last();

        match last_corner {
            None => {}
            Some(last_corner) => segments.push(Segment::new(*last_corner, current_corner)),
        }

        corners.push(current_corner);
    }

    //ajout du dernier segment
    let last_corner = corners.last();
    let first_corner = corners.first();

    match (last_corner, first_corner) {
        (Some(last_corner), Some(first_corner)) => {
            segments.push(Segment::new(*last_corner, *first_corner))
        }
        _ => {
            panic!("corners should not be empty")
        }
    }

    let result_part1 = corners
        .iter()
        .map(|corner1| {
            corners
                .iter()
                .map(|corner2| {
                    let x = (corner1.0 - corner2.0).abs() as i64 + 1;
                    let y = (corner1.1 - corner2.1).abs() as i64 + 1;
                    x * y
                })
                .max()
                .expect("there should be a maximum")
        })
        .max()
        .expect("there should be a maximum");

    println!("result part 1 = {}", result_part1);

    let result_part2 = corners
        .iter()
        .filter_map(|corner1| {
            corners
                .iter()
                .filter_map(|corner3| {

                    //je créée un rectangle legèrement plus petit pour corriger certains cas aux limites
                    let inside_corner_1 = (
                        corner1.0.min(corner3.0) + 0.5,
                        corner1.1.min(corner3.1) + 0.5,
                    );
                    let inside_corner_2 = (
                        corner1.0.max(corner3.0) - 0.5,
                        corner1.1.min(corner3.1) + 0.5,
                    );
                    let inside_corner_3 = (
                        corner1.0.max(corner3.0) - 0.5,
                        corner1.1.max(corner3.1) - 0.5,
                    );
                    let inside_corner_4 = (
                        corner1.0.min(corner3.0) + 0.5,
                        corner1.1.max(corner3.1) - 0.5,
                    );

                    let side1 = Segment::new(inside_corner_1, inside_corner_2);
                    let side2 = Segment::new(inside_corner_2, inside_corner_3);
                    let side3 = Segment::new(inside_corner_3, inside_corner_4);
                    let side4 = Segment::new(inside_corner_4, inside_corner_1);

                    // si il y a au moins un segment de la forme qui entre dans le rectangle, on rejete le rectangle
                    let crossed = segments
                        .iter()
                        .filter(|segment: &&Segment| {
                            segment.cross(&side1)
                                || segment.cross(&side2)
                                || segment.cross(&side3)
                                || segment.cross(&side4)
                        })
                        .peekable()
                        .peek()
                        .is_some();
                    if crossed {
                        return None;
                    };

                    // détection de si on est à l'intérieur ou à l'exterieur de la forme.
                    // on prend un point à l'intérieur du rectangle, on trace un segment jusqu'à un extrémité de la zone, et on compte le nombre d'intersections
                    // si il y en a un nombre impair, on est dans la forme


                    let segment_up = Segment::new(inside_corner_1, (inside_corner_1.0, 0.0));
                    let crossed = segments
                        .iter()
                        .filter(|segment: &&Segment| {
                            segment.cross(&segment_up)
                        })
                        .count();
                    if crossed % 2 == 0 {
                        return None;
                    }

                    let x = (corner1.0 - corner3.0).abs() as i64 + 1;
                    let y = (corner1.1 - corner3.1).abs() as i64 + 1;
                    Some(x * y)
                })
                .max()
        })
        .max()
        .expect("there should be a maximum");

    println!("result part 2 = {}", result_part2);
    Ok(())
}
