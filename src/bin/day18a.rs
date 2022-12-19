#![warn(clippy::pedantic)]

use std::collections::HashSet;
use std::io;
use std::num::ParseIntError;
use std::ops::Add;
use std::str::FromStr;

#[derive(Debug)]
struct DummyError {}

impl From<ParseIntError> for DummyError {
    fn from(_: ParseIntError) -> DummyError {
        DummyError {}
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point(i32, i32, i32);

impl FromStr for Point {
    type Err = DummyError;

    fn from_str(s: &str) -> Result<Point, DummyError> {
        let tokens: Vec<_> = s.split(',').collect();
        match tokens.as_slice() {
            &[x, y, z] => Ok(Point(x.parse()?, y.parse()?, z.parse()?)),
            _ => Err(DummyError {}),
        }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        Point(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

const DELTAS: &[Point] = &[
    Point(1, 0, 0),
    Point(-1, 0, 0),
    Point(0, 1, 0),
    Point(0, -1, 0),
    Point(0, 0, 1),
    Point(0, 0, -1),
];

fn main() {
    let lava: HashSet<Point> = io::stdin()
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    let mut surface_area = 0;
    for l in &lava {
        for d in DELTAS {
            if !lava.contains(&(*l + *d)) {
                surface_area += 1;
            }
        }
    }

    println!("{surface_area}");
}
