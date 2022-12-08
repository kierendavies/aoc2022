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

    let scenic_score = |y, x| {
        let tree_house = *trees.get(y, x).unwrap();

        let viewing_distance = |&(dy, dx)| {
            let view_trees = (1..).map_while(|d| trees.get(y + (d * dy), x + (d * dx)));

            let mut d = 0;
            for t in view_trees {
                d += 1;
                if *t >= tree_house {
                    break;
                }
            }
            d
        };

        const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        DIRECTIONS.iter().map(viewing_distance).product::<usize>()
    };

    let all_coords = (0..height).flat_map(|y| (0..width).map(move |x| (y, x)));

    let best_score = all_coords
        .map(|(y, x)| scenic_score(y as isize, x as isize))
        .max()
        .unwrap();

    println!("{}", best_score);
}
