#![warn(clippy::pedantic)]

use std::collections::HashSet;
use std::io;
use std::num::ParseIntError;
use std::ops::Add;
use std::str::FromStr;

use itertools::Itertools;

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

    let (x_min, x_max) = lava.iter().map(|l| l.0).minmax().into_option().unwrap();
    let (y_min, y_max) = lava.iter().map(|l| l.1).minmax().into_option().unwrap();
    let (z_min, z_max) = lava.iter().map(|l| l.2).minmax().into_option().unwrap();

    let mut surface_area = 0;

    let start = Point(x_min - 1, y_min - 1, z_min - 1);
    let mut outside: HashSet<Point> = HashSet::new();
    outside.insert(start);
    let mut frontier = vec![start];

    while let Some(p) = frontier.pop() {
        for d in DELTAS {
            let q = p + *d;

            if q.0 < x_min - 1
                || q.0 > x_max + 1
                || q.1 < y_min - 1
                || q.1 > y_max + 1
                || q.2 < z_min - 1
                || q.2 > z_max + 1
            {
                continue;
            }

            if lava.contains(&q) {
                surface_area += 1;
            } else if !outside.contains(&q) {
                outside.insert(q);
                frontier.push(q);
            }
        }
    }

    println!("{surface_area}");
}
