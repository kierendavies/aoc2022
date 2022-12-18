#![warn(clippy::pedantic)]

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
const N_ROCKS: usize = 2022;

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
    let mut jets_iter = jets.iter().cycle();
    for rock in ROCKS.iter().cycle().take(N_ROCKS) {
        let mut x = ROCK_X_OFFSET;
        let mut y = chamber.height() + ROCK_Y_OFFSET;

        loop {
            let jet = jets_iter.next().unwrap();
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
    }

    println!("{}", chamber.height());
}
