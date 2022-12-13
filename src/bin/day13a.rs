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

fn main() {
    let mut input = io::stdin().lines().map(Result::unwrap);

    let mut index = 1;
    let mut sum = 0;
    while let Some(left_s) = input.next() {
        if left_s.is_empty() {
            continue;
        }

        let right_s = input.next().unwrap();

        let left = Node::parse(&mut left_s.chars().peekable());
        let right = Node::parse(&mut right_s.chars().peekable());

        if left <= right {
            sum += index;
        }

        index += 1;
    }

    println!("{}", sum);
}
