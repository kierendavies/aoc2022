use std::io;
use std::str::FromStr;

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
}

impl<I> Machine<I>
where
    I: Iterator<Item = Instr>,
{
    fn new(instrs: I) -> Self {
        Machine {
            instrs,
            cycle: 1,
            current_instr: None,
            instr_cycle: 0,
            reg: Registers { x: 1 },
        }
    }

    fn step(&mut self) -> Result<(), ()> {
        if self.current_instr.is_none() {
            self.current_instr = self.instrs.next();
            self.instr_cycle = 1;
        } else {
            self.instr_cycle += 1;
        }

        let current_instr = self.current_instr.as_ref().ok_or(())?;

        self.cycle += 1;

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

    fn signal_strength(&self) -> isize {
        (self.cycle as isize) * self.reg.x
    }
}

fn main() {
    let instrs = io::stdin()
        .lines()
        .map(|l| l.unwrap().parse::<Instr>().unwrap());

    let mut m = Machine::new(instrs);
    println!("{:#?}", m);

    let mut ss = 0;
    while m.step().is_ok() {
        println!("{:#?}", m);
        if m.cycle % 40 == 20 {
            ss += m.signal_strength();
        }
    }

    println!("{}", ss);
}
