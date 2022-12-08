use std::fmt::Debug;
use std::io;
use std::io::BufRead;
use std::io::Read;

struct Matrix<T> {
    vec: Vec<T>,
    width: usize,
}

impl<T> Matrix<T> {
    fn get(&self, y: isize, x: isize) -> Option<&T> {
        if x < 0 || y < 0 {
            return None;
        }

        let x: usize = x.try_into().ok().unwrap();
        let y: usize = y.try_into().ok().unwrap();

        if x >= self.width {
            return None;
        }

        self.vec.get((y * self.width) + x)
    }

    fn set(&mut self, y: isize, x: isize, value: T) {
        assert!(x >= 0 && y >= 0);

        let x: usize = x.try_into().ok().unwrap();
        let y: usize = y.try_into().ok().unwrap();

        assert!(x < self.width);

        self.vec[(y * self.width) + x] = value;
    }
}

impl<T: Debug> Debug for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Matrix {{")?;
        for row in self.vec.chunks_exact(self.width) {
            write!(f, "    ")?;
            for elem in row {
                elem.fmt(f)?;
                write!(f, " ")?;
            }
            writeln!(f)?;
        }
        write!(f, "}}")
    }
}

fn main() {
    let mut stdin = io::stdin().lock();

    let mut first_line = String::new();
    stdin.read_line(&mut first_line).unwrap();

    let width = first_line.trim_end().len();

    let trees = Matrix {
        vec: first_line
            .bytes()
            .chain(stdin.bytes().map(Result::unwrap))
            .filter(|b| !b.is_ascii_whitespace())
            .map(|b| b - b'0')
            .collect(),
        width,
    };

    let height = trees.vec.len() / width;
    assert_eq!(trees.vec.len(), width * height);

    let mut visible = Matrix {
        vec: vec![false; width * height],
        width,
    };

    let mut mark_visible = |y, x, dy, dx| {
        let mut y = y as isize;
        let mut x = x as isize;
        let mut tallest = 0;

        visible.set(y, x, true);

        while let Some(tree) = trees.get(y, x) {
            if *tree > tallest {
                visible.set(y, x, true);
                tallest = *tree;
            }

            y += dy;
            x += dx;
        }
    };

    // Rows
    for y in 0..height {
        // Left to right
        mark_visible(y, 0, 0, 1);
        // Right to left
        mark_visible(y, width - 1, 0, -1);
    }
    // Columns
    for x in 0..width {
        // Top to bottom
        mark_visible(0, x, 1, 0);
        // Bottom to top
        mark_visible(height - 1, x, -1, 0);
    }

    println!("{}", visible.vec.iter().filter(|v| **v).count())
}
