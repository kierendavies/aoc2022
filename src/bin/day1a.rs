use std::error;
use std::io;

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut max = 0;
    let mut curr = 0;

    for line_res in io::stdin().lines() {
        let line = line_res?;
        if line.trim().is_empty() {
            curr = 0;
        } else {
            curr += line.parse::<i32>()?;
        }

        if curr > max {
            max = curr;
        }
    }

    println!("{}", max);
    Ok(())
}
