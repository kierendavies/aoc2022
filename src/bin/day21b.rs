#![warn(clippy::pedantic)]

use std::collections::HashMap;
use std::error;
use std::fmt::Debug;
use std::io;
use std::str;
use std::str::FromStr;

#[derive(Debug)]
struct Error {}

impl<E: error::Error> From<E> for Error {
    fn from(_: E) -> Self {
        Error {}
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Label([char; 4]);

impl Debug for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        String::from_iter(self.0).fmt(f)
    }
}

impl FromStr for Label {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<char> = s.chars().collect();
        let arr = chars.try_into().or(Err(Error {}))?;
        Ok(Label(arr))
    }
}

#[derive(Clone, Copy, Debug)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    fn eval(self, a: i64, b: i64) -> i64 {
        match self {
            Op::Add => a + b,
            Op::Sub => a - b,
            Op::Mul => a * b,
            Op::Div => a / b,
        }
    }
}

impl FromStr for Op {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Op::Add),
            "-" => Ok(Op::Sub),
            "*" => Ok(Op::Mul),
            "/" => Ok(Op::Div),
            _ => Err(Error {}),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Job {
    Num(i64),
    Op { op: Op, a: Label, b: Label },
}

impl FromStr for Job {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().all(|c| c.is_ascii_digit()) {
            Ok(Job::Num(s.parse()?))
        } else {
            let tokens: Vec<_> = s.split_whitespace().collect();
            match tokens.as_slice() {
                &[a, op, b] => Ok(Job::Op {
                    op: op.parse()?,
                    a: a.parse()?,
                    b: b.parse()?,
                }),
                _ => Err(Error {}),
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Monkey {
    label: Label,
    job: Job,
}

impl FromStr for Monkey {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (label, job) = s.split_once(": ").ok_or(Error {})?;

        Ok(Monkey {
            label: label.parse()?,
            job: job.parse()?,
        })
    }
}

const ROOT: Label = Label(['r', 'o', 'o', 't']);
const HUMN: Label = Label(['h', 'u', 'm', 'n']);

fn main() {
    fn eval(jobs: &HashMap<Label, Job>, label: Label) -> Option<i64> {
        if label == HUMN {
            return None;
        }

        match jobs[&label] {
            Job::Num(n) => Some(n),
            Job::Op { op, a, b } => {
                let a_val = eval(jobs, a);
                let b_val = eval(jobs, b);
                a_val.zip(b_val).map(|(a, b)| op.eval(a, b))
            }
        }
    }

    fn constrain_humn_value(jobs: &HashMap<Label, Job>, label: Label, must_eq: i64) -> i64 {
        if label == HUMN {
            return must_eq;
        }

        let Job::Op { op, a, b } = jobs[&label] else { unreachable!() };
        let a_val = eval(jobs, a);
        let b_val = eval(jobs, b);

        let next_label = match (a_val, b_val) {
            (None, Some(_)) => a,
            (Some(_), None) => b,
            (None, None) | (Some(_), Some(_)) => panic!(),
        };

        let next_must_eq = match (op, a_val, b_val) {
            (Op::Add, None, Some(val)) | (Op::Add, Some(val), None) => must_eq - val,
            (Op::Sub, None, Some(val)) => must_eq + val,
            (Op::Sub, Some(val), None) => val - must_eq,
            (Op::Mul, None, Some(val)) | (Op::Mul, Some(val), None) => must_eq / val,
            (Op::Div, None, Some(val)) => must_eq * val,
            (Op::Div, Some(val), None) => val / must_eq,
            (_, None, None) | (_, Some(_), Some(_)) => panic!(),
        };

        constrain_humn_value(jobs, next_label, next_must_eq)
    }

    let monkeys: Vec<Monkey> = io::stdin()
        .lines()
        .map(|line| line.unwrap().parse::<Monkey>().unwrap())
        .collect();

    let jobs: HashMap<Label, Job> = monkeys.iter().map(|m| (m.label, m.job)).collect();

    let Job::Op { op: _, a, b } = jobs[&ROOT] else { panic!() };
    let a_val = eval(&jobs, a);
    let b_val = eval(&jobs, b);
    let humn_value = match (a_val, b_val) {
        (None, Some(val)) => constrain_humn_value(&jobs, a, val),
        (Some(val), None) => constrain_humn_value(&jobs, b, val),
        (None, None) | (Some(_), Some(_)) => panic!(),
    };

    println!("{humn_value}");
}
