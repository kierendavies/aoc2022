#![warn(clippy::pedantic)]

use std::array;
use std::collections::HashMap;
use std::io;

#[derive(Debug)]
enum Jet {
    L,
    R,
}

impl TryFrom<char> for Jet {
    type Error = ();

    fn try_from(c: char) -> Result<Jet, ()> {
        match c {
            '<' => Ok(Jet::L),
            '>' => Ok(Jet::R),
            _ => Err(()),
        }
    }
}

const CHAMBER_WIDTH: usize = 7;
const ROCK_MAX_HEIGHT: usize = 4;

struct Rock {
    bitsets: [u8; ROCK_MAX_HEIGHT],
    width: usize,
    height: usize,
}

#[derive(Debug)]
struct Chamber(Vec<u8>);

impl Chamber {
    fn new() -> Chamber {
        Chamber(Vec::new())
    }

    fn height(&self) -> usize {
        self.0.len()
    }

    fn can_put_rock(&self, rock: &Rock, x: usize, y: usize) -> bool {
        if x + rock.width > CHAMBER_WIDTH {
            return false;
        }

        for dy in 0..rock.height {
            if y + dy >= self.0.len() {
                return true;
            }
            if self.0[y + dy] & (rock.bitsets[dy] << x) != 0 {
                return false;
            }
        }
        true
    }

    fn put_rock(&mut self, rock: &Rock, x: usize, y: usize) {
        if self.0.len() < y + rock.height {
            self.0.resize(y + rock.height, 0);
        }
        for dy in 0..rock.height {
            self.0[y + dy] |= rock.bitsets[dy] << x;
        }
    }

    fn profile(&self) -> [usize; CHAMBER_WIDTH] {
        // 0..CHAMBER_WIDTH.map(|x| 0).into()
        array::from_fn(|x| {
            let mut d = 0;
            while d < self.height() && self.0[self.height() - d - 1] & (1 << x) == 0 {
                d += 1;
            }
            d
        })
    }
}

#[rustfmt::skip]
const ROCKS: &[Rock] = &[
    Rock {
        bitsets: [
            0b1111,
            0b0000,
            0b0000,
            0b0000,
        ],
        width: 4,
        height: 1,
    },
    Rock {
        bitsets: [
            0b010,
            0b111,
            0b010,
            0b000,
        ],
        width: 3,
        height: 3,
    },
    Rock {
        bitsets: [
            0b111,
            0b100,
            0b100,
            0b000,
        ],
        width: 3,
        height: 3,
    },
    Rock {
        bitsets: [
            0b1,
            0b1,
            0b1,
            0b1,
        ],
        width: 1,
        height: 4,
    },
    Rock {
        bitsets: [
            0b11,
            0b11,
            0b00,
            0b00,
        ],
        width: 2,
        height: 2,
    },
];

const ROCK_X_OFFSET: usize = 2;
const ROCK_Y_OFFSET: usize = 3;
const N_ROCKS: usize = 1_000_000_000_000;

fn main() {
    let jets: Vec<Jet> = io::stdin()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .map(|c| c.try_into().unwrap())
        .collect();

    let mut chamber = Chamber::new();
    let mut rocks_iter = ROCKS.iter().enumerate().cycle().enumerate().take(N_ROCKS);
    let mut jets_iter = jets.iter().enumerate().cycle().peekable();
    let mut seen = HashMap::new();
    let mut heights = vec![];

    let (i, cycle_key) = loop {
        let (i, (rock_i, rock)) = rocks_iter.next().unwrap();

        heights.push(chamber.height());

        let jet_i = jets_iter.peek().unwrap().0;
        let profile = chamber.profile();
        let key = (rock_i, jet_i, profile);

        if seen.contains_key(&key) {
            break (i, key);
        }
        seen.insert(key, i);

        let mut x = ROCK_X_OFFSET;
        let mut y = chamber.height() + ROCK_Y_OFFSET;

        loop {
            let jet = jets_iter.next().unwrap().1;
            let new_x = match jet {
                Jet::L if x == 0 => x,
                Jet::L => x - 1,
                Jet::R => x + 1,
            };
            if chamber.can_put_rock(rock, new_x, y) {
                x = new_x;
            }

            if y > 0 && chamber.can_put_rock(rock, x, y - 1) {
                y -= 1;
            } else {
                break;
            }
        }

        chamber.put_rock(rock, x, y);
    };

    let cycle_start_i = seen[&cycle_key];
    let cycle_start_height = heights[cycle_start_i];
    let cycle_len = i - cycle_start_i;
    let cycle_height = chamber.height() - cycle_start_height;
    let n_cycles = (N_ROCKS - cycle_start_i) / cycle_len;
    let rem = (N_ROCKS - cycle_start_i) % cycle_len;
    let rem_height = heights[cycle_start_i + rem] - cycle_start_height;
    let final_height = cycle_start_height + (cycle_height * n_cycles) + rem_height;

    println!("{final_height}");
}
