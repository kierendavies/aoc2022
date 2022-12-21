#![warn(clippy::pedantic)]

use std::cmp::Ordering;
use std::io;

const DECRYPTION_KEY: i64 = 811_589_153;
const MIX_ROUNDS: usize = 10;

fn main() {
    let encrypted: Vec<i64> = io::stdin()
        .lines()
        .map(|line| line.unwrap().parse::<i64>().unwrap() * DECRYPTION_KEY)
        .collect();

    let max_index = encrypted.len() - 1;
    let mut index_orig2new: Vec<usize> = (0..encrypted.len()).collect();
    let mut index_new2orig: Vec<usize> = index_orig2new.clone();

    for _round in 0..MIX_ROUNDS {
        for (i_orig, &v) in encrypted.iter().enumerate() {
            let move_dist = v.unsigned_abs() % (max_index as u64);
            for _ in 0..move_dist {
                let i_new = index_orig2new[i_orig];

                let j_new = match v.cmp(&0) {
                    Ordering::Less if i_new == 0 => max_index,
                    Ordering::Less => i_new - 1,
                    Ordering::Greater if i_new == max_index => 0,
                    Ordering::Greater => i_new + 1,
                    Ordering::Equal => unreachable!(),
                };

                let j_orig = index_new2orig[j_new];

                index_orig2new.swap(i_orig, j_orig);
                index_new2orig.swap(i_new, j_new);
            }
        }
    }

    let mixed: Vec<_> = index_new2orig.iter().map(|&orig| encrypted[orig]).collect();

    let coordinates_sum: i64 = mixed
        .iter()
        .cycle()
        .skip_while(|&&n| n != 0)
        .step_by(1000)
        .skip(1)
        .take(3)
        .sum();

    println!("{coordinates_sum}");
}
