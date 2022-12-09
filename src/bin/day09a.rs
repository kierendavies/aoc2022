use std::collections::HashSet;
use std::io;
use std::ops;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point(isize, isize);

impl Point {
    fn abs_chebyshev(self) -> isize {
        Ord::max(self.0.abs(), self.1.abs())
    }

    fn clamp_chebyshev(self, d: isize) -> Self {
        Self(self.0.clamp(-d, d), self.1.clamp(-d, d))
    }
}

impl ops::Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl ops::AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl ops::Sub for Point {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

fn main() {
    let mut h = Point(0, 0);
    let mut t = Point(0, 0);
    let mut visited: HashSet<Point> = HashSet::new();

    for line in io::stdin().lines().map(Result::unwrap) {
        let tokens: Vec<_> = line.split_whitespace().collect();
        assert_eq!(tokens.len(), 2);
        let dir = match tokens[0] {
            "U" => Point(0, 1),
            "D" => Point(0, -1),
            "L" => Point(-1, 0),
            "R" => Point(1, 0),
            _ => panic!(),
        };
        let dist: usize = tokens[1].parse().unwrap();

        for _ in 0..dist {
            h += dir;

            let d = h - t;
            if d.abs_chebyshev() > 1 {
                t += d.clamp_chebyshev(1);
            }

            visited.insert(t);
        }
    }

    println!("{}", visited.len());
}
