use std::collections::VecDeque;
use std::io;

fn main() {
    let input: Vec<String> = io::stdin().lines().map(Result::unwrap).collect();

    let x_max = input[0].len() - 1;
    let y_max = input.len() - 1;
    for line in &input {
        assert_eq!(line.len(), x_max + 1);
    }

    let find_char = |c| {
        input
            .iter()
            .enumerate()
            .find_map(|(y, line)| line.chars().position(|cc| cc == c).map(|x| (x, y)))
            .unwrap()
    };
    let start = find_char('S');
    let end = find_char('E');

    let elevation: Vec<Vec<u8>> = input
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'S' => 0,
                    'E' => 25,
                    c if ('a'..='z').contains(&c) => (c as u8) - b'a',
                    _ => panic!(),
                })
                .collect()
        })
        .collect();

    let mut visited: Vec<Vec<bool>> = vec![vec![false; x_max + 1]; y_max + 1];
    let mut queue: VecDeque<((usize, usize), u32)> = VecDeque::new();

    visited[start.1][start.0] = true;
    queue.push_back((start, 0));

    let dist = loop {
        let ((x, y), dist) = queue.pop_front().unwrap();

        macro_rules! visit {
            ($x:expr, $y:expr) => {
                if elevation[$y][$x] <= elevation[y][x] + 1 && !visited[$y][$x] {
                    if ($x, $y) == end {
                        break dist + 1;
                    }
                    visited[$y][$x] = true;
                    queue.push_back((($x, $y), dist + 1));
                }
            };
        }

        if x > 0 {
            visit!(x - 1, y);
        }
        if x < x_max {
            visit!(x + 1, y);
        }
        if y > 0 {
            visit!(x, y - 1);
        }
        if y < y_max {
            visit!(x, y + 1);
        }
    };

    println!("{}", dist);
}
