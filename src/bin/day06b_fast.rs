use std::io;

const MARKER_SIZE: usize = 14;

struct Counter {
    counts: [usize; 26],
    distinct: usize,
}

impl Counter {
    fn new() -> Self {
        Self {
            counts: [0; 26],
            distinct: 0,
        }
    }

    fn char_to_index(c: char) -> usize {
        ((c as u8) - b'a') as usize
    }

    fn add(&mut self, c: char) {
        let i = Self::char_to_index(c);
        if self.counts[i] == 0 {
            self.distinct += 1;
        }
        self.counts[i] += 1;
    }

    fn remove(&mut self, c: char) {
        let i = Self::char_to_index(c);
        self.counts[i] -= 1;
        if self.counts[i] == 0 {
            self.distinct -= 1;
        }
    }
}

fn bytes_until_marker(data: Vec<char>) -> Option<usize> {
    let mut ctr = Counter::new();

    for c in &data[..MARKER_SIZE - 1] {
        ctr.add(*c);
    }

    for (offset, window) in data.windows(MARKER_SIZE).enumerate() {
        ctr.add(*window.last().unwrap());

        if ctr.distinct == MARKER_SIZE {
            return Some(offset + MARKER_SIZE);
        }

        ctr.remove(*window.first().unwrap());
    }

    None
}

fn main() {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let data: Vec<char> = input.trim().chars().collect();

    println!("{}", bytes_until_marker(data).unwrap());
}
