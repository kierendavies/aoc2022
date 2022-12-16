#![warn(clippy::pedantic)]

use std::cmp;
use std::io;
use std::ops;
use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Clone, Copy, Debug)]
struct Point {
    x: i64,
    y: i64,
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
    fn abs_manhattan(self) -> i64 {
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
    start: i64,
    end: i64,
}

impl Range {
    fn clamp(self, min: i64, max: i64) -> Range {
        Range {
            start: self.start.max(min),
            end: self.end.min(max),
        }
    }

    fn contains(self, x: i64) -> bool {
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

fn merge_ranges(ranges: Vec<Range>) -> Vec<Range> {
    let mut merged: Vec<Range> = vec![];
    let mut curr = *ranges.first().unwrap();
    for r in ranges {
        if let Some(next) = curr.try_merge(r) {
            curr = next;
        } else {
            merged.push(curr);
            curr = r;
        }
    }
    merged.push(curr);
    merged
}

const XY_MIN: i64 = 0;
const XY_MAX: i64 = 4_000_000;

fn main() {
    let sensors: Vec<Sensor> = io::stdin()
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    for y in XY_MIN..=XY_MAX {
        let mut x_ranges: Vec<Range> = sensors
            .iter()
            .filter_map(|sensor| {
                let radius = (sensor.beacon_pos - sensor.pos).abs_manhattan();
                let dx = radius - (y - sensor.pos.y).abs();
                if dx >= 0 {
                    Some(Range {
                        start: sensor.pos.x - dx,
                        end: sensor.pos.x + dx,
                    })
                } else {
                    None
                }
            })
            .map(|r| r.clamp(XY_MIN, XY_MAX))
            .collect();
        x_ranges.sort_by_key(|r| (r.start, r.end));

        let merged = merge_ranges(x_ranges);

        if merged.len() != 1 {
            let x = merged.first().unwrap().end + 1;
            let tuning_frequency = x * XY_MAX + y;
            println!("{tuning_frequency}");
            break;
        }
    }
}
