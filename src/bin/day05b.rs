use std::io;

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

    let mut temp_stack: Vec<char> = vec![];

    for line in moves_input {
        let tokens: Vec<_> = line.split_whitespace().collect();
        let (n, src, dst) = match tokens.as_slice() {
            &[_, n, _, src, _, dst] => (
                n.parse::<usize>().unwrap(),
                src.parse::<usize>().unwrap() - 1,
                dst.parse::<usize>().unwrap() - 1,
            ),
            _ => panic!("can't parse move: {:?}", line),
        };

        for _ in 0..n {
            let c = stacks[src].pop().unwrap();
            temp_stack.push(c);
        }
        for _ in 0..n {
            let c = temp_stack.pop().unwrap();
            stacks[dst].push(c);
        }
    }

    let tops: String = stacks
        .iter()
        .map(|s| s.last().copied())
        .filter_map(|o| o)
        .collect();

    println!("{}", tops);
}
