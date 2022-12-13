use std::fmt::Debug;
use std::io;
use std::iter::Peekable;

#[derive(Clone, Eq, PartialEq)]
enum Node {
    Int(u32),
    List(Vec<Node>),
}

impl Node {
    fn parse<I: Iterator<Item = char>>(it: &mut Peekable<I>) -> Self {
        if let Some(c) = it.next_if(char::is_ascii_digit) {
            let mut buf = String::from(c);

            while let Some(c) = it.next_if(char::is_ascii_digit) {
                buf.push(c);
            }

            Self::Int(buf.parse().unwrap())
        } else if it.next_if_eq(&'[').is_some() {
            let mut vec = vec![];

            if it.next_if_eq(&']').is_none() {
                vec.push(Self::parse(it));
                while it.next_if_eq(&',').is_some() {
                    vec.push(Self::parse(it));
                }
                assert!(it.next_if_eq(&']').is_some());
            }

            Self::List(vec)
        } else {
            panic!();
        }
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(i) => i.fmt(f),
            Self::List(v) => v.fmt(f),
        }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Node::Int(self_i), Node::Int(other_i)) => self_i.cmp(other_i),
            (Node::Int(_), Node::List(_)) => Node::List(vec![self.clone()]).cmp(other),
            (Node::List(_), Node::Int(_)) => self.cmp(&Node::List(vec![other.clone()])),
            (Node::List(self_vec), Node::List(other_vec)) => self_vec.cmp(other_vec),
        }
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

macro_rules! node {
    ($i:literal) => {
        Node::Int($i)
    };
    ([$($n:tt),*]) => {
        Node::List(vec![$(node!($n)),*])
    };
}

fn main() {
    let mut packets: Vec<Node> = io::stdin()
        .lines()
        .map(Result::unwrap)
        .filter(|line| !line.is_empty())
        .map(|line| Node::parse(&mut line.chars().peekable()))
        .collect();

    let div_1 = node!([[2]]);
    let div_2 = node!([[6]]);

    packets.push(div_1.clone());
    packets.push(div_2.clone());
    packets.sort();

    let index_div_1 = packets.binary_search(&div_1).unwrap() + 1;
    let index_div_2 = packets.binary_search(&div_2).unwrap() + 1;
    let key = index_div_1 * index_div_2;

    println!("{:#?}", key);
}
