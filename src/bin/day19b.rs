#![warn(clippy::pedantic)]

use std::cmp::Ordering;
use std::collections::HashSet;
use std::io;
use std::num::ParseIntError;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Index;
use std::ops::IndexMut;
use std::ops::Sub;
use std::ops::SubAssign;
use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
enum Error {
    MatchError,
    ParseIntError(ParseIntError),
}

impl From<ParseIntError> for Error {
    fn from(value: ParseIntError) -> Self {
        Error::ParseIntError(value)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

const RESOURCES: &[Resource] = &[
    Resource::Ore,
    Resource::Clay,
    Resource::Obsidian,
    Resource::Geode,
];

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
struct ResourceMap<T> {
    ore: T,
    clay: T,
    obsidian: T,
    geode: T,
}

impl<T: Add> Add for ResourceMap<T> {
    type Output = ResourceMap<T::Output>;

    fn add(self, rhs: Self) -> Self::Output {
        ResourceMap {
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obsidian: self.obsidian + rhs.obsidian,
            geode: self.geode + rhs.geode,
        }
    }
}

impl<T: Sub> Sub for ResourceMap<T> {
    type Output = ResourceMap<T::Output>;

    fn sub(self, rhs: Self) -> Self::Output {
        ResourceMap {
            ore: self.ore - rhs.ore,
            clay: self.clay - rhs.clay,
            obsidian: self.obsidian - rhs.obsidian,
            geode: self.geode - rhs.geode,
        }
    }
}

impl<T: AddAssign> AddAssign for ResourceMap<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.ore += rhs.ore;
        self.clay += rhs.clay;
        self.obsidian += rhs.obsidian;
        self.geode += rhs.geode;
    }
}

impl<T: SubAssign> SubAssign for ResourceMap<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.ore -= rhs.ore;
        self.clay -= rhs.clay;
        self.obsidian -= rhs.obsidian;
        self.geode -= rhs.geode;
    }
}

impl<T: PartialOrd> PartialOrd for ResourceMap<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use Ordering::Equal;
        use Ordering::Greater;
        use Ordering::Less;

        let ore_ord = self.ore.partial_cmp(&other.ore)?;
        let clay_ord = self.clay.partial_cmp(&other.clay)?;
        let obsidian_ord = self.obsidian.partial_cmp(&other.obsidian)?;
        let geode_ord = self.geode.partial_cmp(&other.geode)?;

        let min_ord = ore_ord.min(clay_ord).min(obsidian_ord).min(geode_ord);
        let max_ord = ore_ord.max(clay_ord).max(obsidian_ord).max(geode_ord);

        match (min_ord, max_ord) {
            (Less, Less | Equal) => Some(Less),
            (Equal, Equal) => Some(Equal),
            (Equal | Greater, Greater) => Some(Greater),
            (Less, Greater) => None,
            _ => unreachable!(),
        }
    }
}

impl<T> Index<Resource> for ResourceMap<T> {
    type Output = T;

    fn index(&self, index: Resource) -> &Self::Output {
        match index {
            Resource::Ore => &self.ore,
            Resource::Clay => &self.clay,
            Resource::Obsidian => &self.obsidian,
            Resource::Geode => &self.geode,
        }
    }
}

impl<T> IndexMut<Resource> for ResourceMap<T> {
    fn index_mut(&mut self, index: Resource) -> &mut Self::Output {
        match index {
            Resource::Ore => &mut self.ore,
            Resource::Clay => &mut self.clay,
            Resource::Obsidian => &mut self.obsidian,
            Resource::Geode => &mut self.geode,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct State {
    robots: ResourceMap<u32>,
    resources: ResourceMap<u32>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            robots: ResourceMap {
                ore: 1,
                ..Default::default()
            },
            resources: ResourceMap::default(),
        }
    }
}

#[derive(Debug)]
struct Blueprint {
    _id: u32,
    robot_costs: ResourceMap<ResourceMap<u32>>,
}

impl FromStr for Blueprint {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                "^Blueprint (?P<id>\\d+): \
                    Each ore robot costs (?P<ore_ore>\\d+) ore. \
                    Each clay robot costs (?P<clay_ore>\\d+) ore. \
                    Each obsidian robot costs (?P<obsidian_ore>\\d+) ore and (?P<obsidian_clay>\\d+) clay. \
                    Each geode robot costs (?P<geode_ore>\\d+) ore and (?P<geode_obsidian>\\d+) obsidian.$"
            )
            .unwrap();
        }

        let m = RE.captures(s).ok_or(Error::MatchError)?;

        Ok(Blueprint {
            _id: m["id"].parse()?,
            robot_costs: ResourceMap {
                ore: ResourceMap {
                    ore: m["ore_ore"].parse()?,
                    ..Default::default()
                },
                clay: ResourceMap {
                    ore: m["clay_ore"].parse()?,
                    ..Default::default()
                },
                obsidian: ResourceMap {
                    ore: m["obsidian_ore"].parse()?,
                    clay: m["obsidian_clay"].parse()?,
                    ..Default::default()
                },
                geode: ResourceMap {
                    ore: m["geode_ore"].parse()?,
                    obsidian: m["geode_obsidian"].parse()?,
                    ..Default::default()
                },
            },
        })
    }
}

const TIME_LIMIT: usize = 32;

impl Blueprint {
    fn max_geodes(&self) -> u32 {
        let mut states: Vec<HashSet<State>> = vec![HashSet::new(); TIME_LIMIT + 1];
        states[0].insert(State::default());

        for t in 1..TIME_LIMIT {
            let (states_l, states_r) = states.split_at_mut(t);
            for &state in &states_l[t - 1] {
                'robot: for &robot in RESOURCES {
                    let mut next_state = state;

                    let mut t2 = t;
                    while !matches!(
                        next_state.resources.partial_cmp(&self.robot_costs[robot]),
                        Some(Ordering::Greater | Ordering::Equal)
                    ) {
                        t2 += 1;
                        if t2 > TIME_LIMIT {
                            continue 'robot;
                        }

                        next_state.resources += state.robots;
                    }

                    next_state.resources += state.robots;
                    next_state.resources -= self.robot_costs[robot];
                    next_state.robots[robot] += 1;

                    states_r[t2 - t].insert(next_state);
                }
            }

            states[t - 1].clear();
            states[t - 1].shrink_to_fit();
        }

        let (states_l, states_r) = states.split_at_mut(TIME_LIMIT);
        for &state in &states_l[TIME_LIMIT - 1] {
            let mut next_state = state;
            next_state.resources += state.robots;
            states_r[0].insert(next_state);
        }

        states[TIME_LIMIT]
            .iter()
            .map(|s| s.resources.geode)
            .max()
            .unwrap()
    }
}

const N_BLUEPRINTS: usize = 3;

fn main() {
    let blueprints: Vec<Blueprint> = io::stdin()
        .lines()
        .take(N_BLUEPRINTS)
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    let geodes_product: u32 = blueprints.iter().map(Blueprint::max_geodes).product();

    println!("{geodes_product}");
}
