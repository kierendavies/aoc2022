use std::io;
use std::str;

const PART: usize = 1;

#[derive(Debug)]
struct Move {
    n: usize,
    src: usize,
    dst: usize,
}

impl str::FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<_> = s.split_whitespace().collect();
        match tokens.as_slice() {
            &[_, n_str, _, src_str, _, dst_str] => {
                let n: usize = n_str.parse().or(Err(()))?;
                let src: usize = src_str.parse().or(Err(()))?;
                let dst: usize = dst_str.parse().or(Err(()))?;
                Ok(Move {
                    n,
                    src: src - 1,
                    dst: dst - 1,
                })
            }
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct CratePos {
    stack: usize,
    depth: usize,
}

fn main() {
    let input: Vec<_> = io::stdin().lines().map(Result::unwrap).collect();
    let input_sections: Vec<_> = input.splitn(2, String::is_empty).collect();

    let (stacks_input, moves_input) = match input_sections.as_slice() {
        &[stacks_input, moves_input] => (stacks_input, moves_input),
        _ => panic!("wrong number of sections"),
    };

    let (stacks_labels, stacks_contents) = stacks_input.split_last().unwrap();
    let n_stacks = stacks_labels.split_whitespace().count();
    let mut stacks: Vec<Vec<char>> = vec![vec![]; n_stacks];

    for line in stacks_contents.iter().rev() {
        for (i, c) in line.chars().skip(1).step_by(4).enumerate() {
            if c != ' ' {
                stacks[i].push(c);
            }
        }
    }
    let stacks = stacks;

    let moves: Vec<Move> = moves_input.iter().map(|m| m.parse().unwrap()).collect();

    let mut eventual_tops: Vec<CratePos> = (0..n_stacks)
        .map(|i| CratePos { stack: i, depth: 0 })
        .collect();

    for m in moves.iter().rev() {
        if m.src == m.dst {
            panic!("src == dst")
        }
        for cr in eventual_tops.iter_mut() {
            if cr.stack == m.src {
                cr.depth += m.n;
            }
            if cr.stack == m.dst {
                if cr.depth >= m.n {
                    cr.depth -= m.n;
                } else {
                    cr.stack = m.src;
                    if PART == 1 {
                        cr.depth = m.n - cr.depth - 1;
                    }
                }
            }
        }
    }

    let tops: String = eventual_tops
        .iter()
        .filter_map(|cr| {
            let s = &stacks[cr.stack];
            if cr.depth < s.len() {
                Some(s[s.len() - cr.depth - 1])
            } else {
                None
            }
        })
        .collect();

    println!("{}", tops);
}
