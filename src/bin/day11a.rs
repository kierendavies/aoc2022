use std::vec;

struct Monkey<'a> {
    items: Vec<u32>,
    op: &'a dyn Fn(u32) -> u32,
    test_div: u32,
    if_true: usize,
    if_false: usize,
    inspections: u32,
}

fn main() {
    // Ugh, I don't want to write a parser for this.
    let monkeys: &mut [Monkey] = &mut [
        Monkey {
            items: vec![54, 53],
            op: &|w| w * 3,
            test_div: 2,
            if_true: 2,
            if_false: 6,
            inspections: 0,
        },
        Monkey {
            items: vec![95, 88, 75, 81, 91, 67, 65, 84],
            op: &|w| w * 11,
            test_div: 7,
            if_true: 3,
            if_false: 4,
            inspections: 0,
        },
        Monkey {
            items: vec![76, 81, 50, 93, 96, 81, 83],
            op: &|w| w + 6,
            test_div: 3,
            if_true: 5,
            if_false: 1,
            inspections: 0,
        },
        Monkey {
            items: vec![83, 85, 85, 63],
            op: &|w| w + 4,
            test_div: 11,
            if_true: 7,
            if_false: 4,
            inspections: 0,
        },
        Monkey {
            items: vec![85, 52, 64],
            op: &|w| w + 8,
            test_div: 17,
            if_true: 0,
            if_false: 7,
            inspections: 0,
        },
        Monkey {
            items: vec![57],
            op: &|w| w + 2,
            test_div: 5,
            if_true: 1,
            if_false: 3,
            inspections: 0,
        },
        Monkey {
            items: vec![60, 95, 76, 66, 91],
            op: &|w| w * w,
            test_div: 13,
            if_true: 2,
            if_false: 5,
            inspections: 0,
        },
        Monkey {
            items: vec![65, 84, 76, 72, 79, 65],
            op: &|w| w + 5,
            test_div: 19,
            if_true: 6,
            if_false: 0,
            inspections: 0,
        },
    ];

    for _round in 0..20 {
        for turn in 0..monkeys.len() {
            let items = monkeys[turn].items.clone();
            for mut item in items {
                item = (monkeys[turn].op)(item);
                item /= 3;
                let throw_to = if item % monkeys[turn].test_div == 0 {
                    monkeys[turn].if_true
                } else {
                    monkeys[turn].if_false
                };
                monkeys[throw_to].items.push(item);
            }
            monkeys[turn].inspections += monkeys[turn].items.len() as u32;
            monkeys[turn].items.clear();
        }
    }

    let mut inspections: Vec<u32> = monkeys.iter().map(|m| m.inspections).collect();
    inspections.sort_by(|a, b| b.cmp(a));
    let monkey_business = match inspections.as_slice() {
        [a, b, ..] => a * b,
        _ => unreachable!(),
    };
    println!("{}", monkey_business);
}
