use std::collections::HashSet;
use std::io;

fn main() {
    let mut sum = 0;

    for line_res in io::stdin().lines() {
        let line = line_res.unwrap();
        let items = line.trim();
        let (l, r) = items.split_at(items.len() / 2);

        let l_set: HashSet<char> = l.chars().collect();
        let r_set: HashSet<char> = r.chars().collect();
        let dup = *l_set.intersection(&r_set).next().unwrap();

        let prio = if 'a' <= dup && dup <= 'z' {
            (dup as i32) - ('a' as i32) + 1
        } else if 'A' <= dup && dup <= 'Z' {
            (dup as i32) - ('A' as i32) + 27
        } else {
            panic!()
        };

        sum += prio;
    }

    println!("{}", sum);
}
