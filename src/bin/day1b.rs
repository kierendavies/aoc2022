use std::error;
use std::io;

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut cals: Vec<i32> = vec![];

    let mut curr = 0;
    for line_res in io::stdin().lines() {
        let line = line_res?;
        if line.trim().is_empty() {
            cals.push(curr);
            curr = 0;
        } else {
            curr += line.parse::<i32>()?;
        }
    }
    cals.push(curr);

    cals.sort();
    let top3: i32 = cals[cals.len() - 3..].iter().sum();

    println!("{}", top3);
    Ok(())
}
