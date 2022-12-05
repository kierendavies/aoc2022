use core::fmt;
use std::error;
use std::io;
use std::str;

#[derive(Debug)]
struct ParseError {
    input: String,
}

impl ParseError {
    fn err<T>(input: &str) -> Result<T, ParseError> {
        Err(ParseError {
            input: input.to_owned(),
        })
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ParseError({:?})", self.input)
    }
}

impl error::Error for ParseError {}

#[derive(Debug)]
struct Assignment {
    start: i32,
    end: i32,
}

impl Assignment {
    fn contains(&self, other: &Assignment) -> bool {
        self.start <= other.start && other.end <= self.end
    }
}

impl str::FromStr for Assignment {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split('-').collect();
        match parts.as_slice() {
            &[start_str, end_str] => {
                let start = start_str.parse().or(ParseError::err(s))?;
                let end = end_str.parse().or(ParseError::err(s))?;
                Ok(Assignment { start, end })
            }
            _ => ParseError::err(s),
        }
    }
}

#[derive(Debug)]
struct Pair(Assignment, Assignment);

impl str::FromStr for Pair {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(',').collect();
        match parts.as_slice() {
            &[l_str, r_str] => {
                let l = l_str.parse().or(ParseError::err(s))?;
                let r = r_str.parse().or(ParseError::err(s))?;
                Ok(Pair(l, r))
            }
            _ => ParseError::err(s),
        }
    }
}

fn main() {
    let input = io::stdin().lines().map(Result::unwrap);
    let pairs = input.map(|line| line.parse::<Pair>().unwrap());
    let overlapping = pairs.filter(|Pair(l, r)| l.contains(r) || r.contains(l));
    println!("{}", overlapping.count());
}
