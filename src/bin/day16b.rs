#![warn(clippy::pedantic)]

use std::collections::HashMap;
use std::fmt::Debug;
use std::io;
use std::str;
use std::str::FromStr;
use std::vec;

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Label([u8; 2]);

impl Debug for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        str::from_utf8(&self.0).unwrap().fmt(f)
    }
}

impl FromStr for Label {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.as_bytes() {
            &[b0, b1] => Ok(Label([b0, b1])),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Valve {
    label: Label,
    flow_rate: u32,
    neighbours: Vec<Label>,
}

impl FromStr for Valve {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"^Valve (?P<label>[A-Z]{2}) has flow rate=(?P<flow_rate>\d+); tunnels? leads? to valves? (?P<neighbours>[A-Z]{2}(?:, [A-Z]{2})*)$"
            ).unwrap();
        }

        let m = RE.captures(s).ok_or(())?;

        Ok(Valve {
            label: m["label"].parse()?,
            flow_rate: m["flow_rate"].parse().or(Err(()))?,
            neighbours: m["neighbours"]
                .split(", ")
                .map(str::parse)
                .collect::<Result<_, _>>()?,
        })
    }
}

#[derive(Clone)]
struct SearchActorState {
    at: Label,
    opened: Vec<Label>,
    remaining_time: u32,
}

#[derive(Clone)]
struct SearchState {
    you: SearchActorState,
    elephant: SearchActorState,
    flow: u32,
}

const START_LABEL: Label = Label(*b"AA");
const TIME_LIMIT: u32 = 26;

fn main() {
    let valves: Vec<Valve> = io::stdin()
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    let mut dist: HashMap<(Label, Label), u32> = HashMap::new();
    for v in &valves {
        dist.insert((v.label, v.label), 0);
        for w in &v.neighbours {
            dist.insert((v.label, *w), 1);
        }
    }
    for hop in &valves {
        for v in &valves {
            for w in &valves {
                if let (Some(dist_v_hop), Some(dist_hop_k)) = (
                    dist.get(&(v.label, hop.label)),
                    dist.get(&(hop.label, w.label)),
                ) {
                    let new_dist = dist_v_hop + dist_hop_k;
                    dist.entry((v.label, w.label))
                        .and_modify(|d| {
                            if new_dist < *d {
                                *d = new_dist;
                            }
                        })
                        .or_insert(new_dist);
                }
            }
        }
    }
    let dist = dist;

    let nonzero_flow_rates: HashMap<Label, u32> = valves
        .iter()
        .filter(|v| v.flow_rate != 0)
        .map(|v| (v.label, v.flow_rate))
        .collect();

    let mut max_flow = 0;
    let mut stack = vec![SearchState {
        you: SearchActorState {
            at: START_LABEL,
            opened: Vec::new(),
            remaining_time: TIME_LIMIT,
        },
        elephant: SearchActorState {
            at: START_LABEL,
            opened: Vec::new(),
            remaining_time: TIME_LIMIT,
        },
        flow: 0,
    }];
    while let Some(curr) = stack.pop() {
        for (w, flow_rate) in &nonzero_flow_rates {
            if curr.you.opened.contains(w) || curr.elephant.opened.contains(w) {
                continue;
            }

            if curr.you.remaining_time >= curr.elephant.remaining_time {
                // You
                if let Some(remaining_time) = curr
                    .you
                    .remaining_time
                    .checked_sub(dist[&(curr.you.at, *w)] + 1)
                {
                    let mut next = curr.clone();
                    next.you.at = *w;
                    next.you.opened.push(*w);
                    next.you.remaining_time = remaining_time;
                    next.flow = curr.flow + flow_rate * remaining_time;

                    if next.flow > max_flow {
                        max_flow = next.flow;
                    }

                    stack.push(next);
                };
            } else {
                // Elephant
                if let Some(remaining_time) = curr
                    .elephant
                    .remaining_time
                    .checked_sub(dist[&(curr.elephant.at, *w)] + 1)
                {
                    let mut next = curr.clone();
                    next.elephant.at = *w;
                    next.elephant.opened.push(*w);
                    next.elephant.remaining_time = remaining_time;
                    next.flow = curr.flow + flow_rate * remaining_time;

                    if next.flow > max_flow {
                        max_flow = next.flow;
                    }

                    stack.push(next);
                };
            }
        }
    }

    println!("{max_flow}");
}
