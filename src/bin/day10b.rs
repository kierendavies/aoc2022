use std::fmt::Debug;
use std::io;
use std::str;
use std::str::FromStr;

const DISP_WIDTH: usize = 40;
const DISP_HEIGHT: usize = 6;

#[derive(Debug)]
enum Instr {
    NoOp,
    AddX { v: isize },
}

impl Instr {
    fn cycles(&self) -> usize {
        match self {
            Instr::NoOp => 1,
            Instr::AddX { v: _ } => 2,
        }
    }
}

impl FromStr for Instr {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<_> = s.split_whitespace().collect();
        match *tokens.as_slice() {
            [op] if op == "noop" => Ok(Instr::NoOp),
            [op, v] if op == "addx" => Ok(Instr::AddX {
                v: v.parse().map_err(|_| ())?,
            }),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Registers {
    x: isize,
}

#[derive(Debug)]
struct Machine<I>
where
    I: Iterator<Item = Instr>,
{
    instrs: I,
    cycle: usize,
    current_instr: Option<Instr>,
    instr_cycle: usize,
    reg: Registers,
    disp: [[u8; DISP_WIDTH]; DISP_HEIGHT],
}

impl<I> Machine<I>
where
    I: Iterator<Item = Instr>,
{
    fn new(instrs: I) -> Self {
        Machine {
            instrs,
            cycle: 0,
            current_instr: None,
            instr_cycle: 0,
            reg: Registers { x: 1 },
            disp: [[b'?'; DISP_WIDTH]; DISP_HEIGHT],
        }
    }

    fn step(&mut self) -> Result<(), ()> {
        // Load instruction
        if self.current_instr.is_none() {
            self.current_instr = self.instrs.next();
            self.instr_cycle = 0;
        }
        let current_instr = self.current_instr.as_ref().ok_or(())?;

        // Update display
        let disp_x = self.cycle % DISP_WIDTH;
        let disp_y = self.cycle / DISP_WIDTH;
        self.disp[disp_y][disp_x] = if self.reg.x.abs_diff(disp_x as isize) <= 1 {
            b'#'
        } else {
            b'.'
        };

        // Execute instruction
        self.cycle += 1;
        self.instr_cycle += 1;
        if self.instr_cycle == current_instr.cycles() {
            match current_instr {
                Instr::NoOp => {}
                Instr::AddX { v } => {
                    self.reg.x += v;
                }
            }
            self.current_instr = None;
        }

        Ok(())
    }

    fn render_disp(&self) -> String {
        let mut vec: Vec<u8> = vec![];
        for row in self.disp {
            vec.extend_from_slice(&row);
            vec.push(b'\n');
        }
        String::from_utf8(vec).unwrap()
    }
}

fn main() {
    let instrs = io::stdin()
        .lines()
        .map(|l| l.unwrap().parse::<Instr>().unwrap());

    let mut m = Machine::new(instrs);

    while m.step().is_ok() {}

    print!("{}", m.render_disp());
}
