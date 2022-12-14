use std::cmp;
use std::collections::HashMap;
use std::io;
use std::ops;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point(i32, i32);

impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(',') {
            Some((x_str, y_str)) => {
                let x = x_str.parse().or(Err(()))?;
                let y = y_str.parse().or(Err(()))?;
                Ok(Self(x, y))
            }
            None => Err(()),
        }
    }
}
impl ops::Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[derive(Debug)]
struct Rock(Vec<Point>);

impl FromStr for Rock {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let points: Result<_, _> = s.split(" -> ").map(Point::from_str).collect();
        Ok(Rock(points?))
    }
}

#[derive(Eq, PartialEq)]
enum Filled {
    Rock,
    Sand,
}

const SAND_SOURCE: Point = Point(500, 0);
const FALL_DELTAS: [Point; 3] = [Point(0, 1), Point(-1, 1), Point(1, 1)];

fn main() {
    let rocks: Vec<Rock> = io::stdin()
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    let mut cave: HashMap<Point, Filled> = HashMap::new();
    for rock in &rocks {
        assert!(rock.0.len() >= 2);
        for window in rock.0.windows(2) {
            let (p1, p2) = match window {
                [p1, p2] => (p1, p2),
                _ => unreachable!(),
            };

            let x_min = cmp::min(p1.0, p2.0);
            let x_max = cmp::max(p1.0, p2.0);
            let y_min = cmp::min(p1.1, p2.1);
            let y_max = cmp::max(p1.1, p2.1);

            for x in x_min..=x_max {
                for y in y_min..=y_max {
                    cave.insert(Point(x, y), Filled::Rock);
                }
            }
        }
    }

    let y_max = cave.keys().map(|p| p.1).max().unwrap();

    'outer: loop {
        let mut sand = SAND_SOURCE;
        while let Some(sand_next) = FALL_DELTAS
            .iter()
            .map(|d| sand + *d)
            .find(|p| !cave.contains_key(p))
        {
            sand = sand_next;
            if sand.1 >= y_max {
                break 'outer;
            }
        }
        assert_ne!(sand, SAND_SOURCE);
        cave.insert(sand, Filled::Sand);
    }

    let sand_count = cave.values().filter(|f| **f == Filled::Sand).count();
    println!("{sand_count}");
}
