#![warn(clippy::pedantic)]

use std::cmp;
use std::io;
use std::ops;
use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl ops::Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Point {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Point {
    fn abs_manhattan(self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

#[derive(Debug)]
struct Sensor {
    pos: Point,
    beacon_pos: Point,
}

impl FromStr for Sensor {
    type Err = ();

    fn from_str(s: &str) -> Result<Sensor, ()> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^Sensor at x=(?P<sx>-?\d+), y=(?P<sy>-?\d+): closest beacon is at x=(?P<bx>-?\d+), y=(?P<by>-?\d+)$").unwrap();
        }

        let m = RE.captures(s).ok_or(())?;

        Ok(Sensor {
            pos: Point {
                x: m["sx"].parse().or(Err(()))?,
                y: m["sy"].parse().or(Err(()))?,
            },
            beacon_pos: Point {
                x: m["bx"].parse().or(Err(()))?,
                y: m["by"].parse().or(Err(()))?,
            },
        })
    }
}

#[derive(Clone, Copy, Debug)]
struct Range {
    start: i32,
    end: i32,
}

impl Range {
    fn len(self) -> i32 {
        self.end - self.start + 1
    }

    fn contains(self, x: i32) -> bool {
        self.start <= x && x <= self.end
    }

    fn overlaps(self, other: Range) -> bool {
        self.contains(other.start) || other.contains(self.start)
    }

    fn try_merge(self, other: Range) -> Option<Range> {
        if self.overlaps(other) {
            Some(Range {
                start: cmp::min(self.start, other.start),
                end: cmp::max(self.end, other.end),
            })
        } else {
            None
        }
    }
}

const COVERAGE_Y: i32 = 2_000_000;

fn main() {
    let sensors: Vec<Sensor> = io::stdin()
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    let mut x_ranges: Vec<Range> = sensors
        .iter()
        .filter_map(|sensor| {
            let radius = (sensor.beacon_pos - sensor.pos).abs_manhattan();
            let dx = radius - (COVERAGE_Y - sensor.pos.y).abs();
            if dx >= 0 {
                Some(Range {
                    start: sensor.pos.x - dx,
                    end: sensor.pos.x + dx,
                })
            } else {
                None
            }
        })
        .collect();
    x_ranges.sort_by_key(|r| (r.start, r.end));

    let mut merged: Vec<Range> = vec![];
    let mut curr = *x_ranges.first().unwrap();
    for r in x_ranges {
        if let Some(next) = curr.try_merge(r) {
            curr = next;
        } else {
            merged.push(curr);
            curr = r;
        }
    }
    merged.push(curr);

    let coverage: i32 = merged.iter().map(|r| r.len()).sum();

    let mut covered_beacon_xs: Vec<_> = sensors
        .iter()
        .filter(|s| s.beacon_pos.y == COVERAGE_Y)
        .map(|s| s.beacon_pos.x)
        .collect();
    covered_beacon_xs.sort_unstable();
    covered_beacon_xs.dedup();
    let covered_beacons: i32 = covered_beacon_xs.len().try_into().unwrap();

    let no_beacons = coverage - covered_beacons;

    println!("{no_beacons}");
}
