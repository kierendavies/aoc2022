use std::io;

const MARKER_SIZE: usize = 14;

fn main() {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let data: Vec<char> = input.trim().chars().collect();

    let (offset, _) = data
        .windows(MARKER_SIZE)
        .enumerate()
        .find(|(_, window)| {
            let mut distinct = window.to_vec();
            distinct.sort();
            distinct.dedup();
            distinct.len() == MARKER_SIZE
        })
        .unwrap();

    println!("{}", offset + MARKER_SIZE);
}
