use std::error;
use std::io;

#[derive(Clone, Copy, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(PartialEq)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

const BEATS: &[(Shape, Shape)] = &[
    (Shape::Rock, Shape::Scissors),
    (Shape::Paper, Shape::Rock),
    (Shape::Scissors, Shape::Paper),
];

fn parse_line(line: &str) -> Option<(Shape, Outcome)> {
    let opp = match line.chars().nth(0)? {
        'A' => Some(Shape::Rock),
        'B' => Some(Shape::Paper),
        'C' => Some(Shape::Scissors),
        _ => None,
    }?;

    let outcome = match line.chars().nth(2)? {
        'X' => Some(Outcome::Lose),
        'Y' => Some(Outcome::Draw),
        'Z' => Some(Outcome::Win),
        _ => None,
    }?;

    Some((opp, outcome))
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut score = 0;

    for line_res in io::stdin().lines() {
        let line = line_res?;
        let (opp, outcome) = parse_line(line.as_str()).ok_or("could not parse")?;

        let me = match outcome {
            Outcome::Lose => BEATS.iter().find(|(s, _)| *s == opp).unwrap().1,
            Outcome::Draw => opp,
            Outcome::Win => BEATS.iter().find(|(_, s)| *s == opp).unwrap().0,
        };

        let shape_score = match me {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        };

        let outcome_score = match outcome {
            Outcome::Lose => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        };

        score += shape_score + outcome_score;
    }

    println!("{}", score);
    Ok(())
}
