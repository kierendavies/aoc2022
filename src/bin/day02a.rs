use std::error;
use std::io;

#[derive(Clone, Copy, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

fn parse_line(line: &str) -> Option<(Shape, Shape)> {
    let opp = match line.chars().nth(0)? {
        'A' => Some(Shape::Rock),
        'B' => Some(Shape::Paper),
        'C' => Some(Shape::Scissors),
        _ => None,
    }?;

    let me = match line.chars().nth(2)? {
        'X' => Some(Shape::Rock),
        'Y' => Some(Shape::Paper),
        'Z' => Some(Shape::Scissors),
        _ => None,
    }?;

    Some((opp, me))
}

fn beats(a: Shape, b: Shape) -> bool {
    match (a, b) {
        (Shape::Rock, Shape::Scissors) => true,
        (Shape::Paper, Shape::Rock) => true,
        (Shape::Scissors, Shape::Paper) => true,
        _ => false,
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut score = 0;

    for line_res in io::stdin().lines() {
        let line = line_res?;
        let (opp, me) = parse_line(line.as_str()).ok_or("could not parse")?;

        let shape_score = match me {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        };

        let outcome_score = if beats(me, opp) {
            6
        } else if me == opp {
            3
        } else {
            0
        };

        score += shape_score + outcome_score;
    }

    println!("{}", score);
    Ok(())
}
