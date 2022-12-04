use std::collections::HashSet;
use std::io;

fn main() {
    let rucksacks: Vec<HashSet<char>> = io::stdin()
        .lines()
        .map(|line_res| {
            let line = line_res.unwrap();
            line.trim().chars().collect()
        })
        .collect();

    let badges = rucksacks.chunks(3).map(|group| {
        let mut intersection = group.first().unwrap().clone();
        for r in group {
            intersection.retain(|item| r.contains(item));
        }
        *intersection.iter().next().unwrap()
    });

    let prios = badges.map(|item| {
        if 'a' <= item && item <= 'z' {
            (item as i32) - ('a' as i32) + 1
        } else if 'A' <= item && item <= 'Z' {
            (item as i32) - ('A' as i32) + 27
        } else {
            panic!()
        }
    });

    let sum: i32 = prios.sum();

    println!("{}", sum);
}
