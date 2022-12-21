#![warn(clippy::pedantic)]

use std::cmp::Ordering;
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
    remaining_time: u32,
    robots: ResourceMap<u32>,
    resources: ResourceMap<u32>,
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

const BUILD_PRIORITY: &[Resource] = &[
    Resource::Geode,
    Resource::Obsidian,
    Resource::Clay,
    Resource::Ore,
];

impl Blueprint {
    fn max_geodes(&self, time_limit: u32) -> u32 {
        fn upper_bound(state: State) -> u32 {
            // Already collected
            state.resources.geode
            // To be collected by current robots
                + (state.robots.geode * state.remaining_time)
            // If we build a geode robot every minute
                + ((state.remaining_time * (state.remaining_time - 1)) / 2)
        }

        fn dfs(blueprint: &Blueprint, state: State) -> u32 {
            if state.remaining_time == 0 {
                return state.resources.geode;
            }

            if state.remaining_time == 1 {
                return state.resources.geode + state.robots.geode;
            }

            let mut max_geodes = state.resources.geode;

            'robot: for &robot in BUILD_PRIORITY {
                let mut next_state = state;

                while !matches!(
                    next_state
                        .resources
                        .partial_cmp(&blueprint.robot_costs[robot]),
                    Some(Ordering::Greater | Ordering::Equal)
                ) {
                    next_state.remaining_time -= 1;
                    if next_state.remaining_time == 0 {
                        continue 'robot;
                    }
                    next_state.resources += state.robots;
                }

                next_state.remaining_time -= 1;
                next_state.resources += state.robots;
                next_state.resources -= blueprint.robot_costs[robot];
                next_state.robots[robot] += 1;

                if upper_bound(next_state) <= max_geodes {
                    continue;
                }

                let geodes = dfs(blueprint, next_state);
                if geodes > max_geodes {
                    max_geodes = geodes;
                }
            }

            max_geodes
        }

        let init_state = State {
            remaining_time: time_limit,
            robots: ResourceMap {
                ore: 1,
                ..Default::default()
            },
            resources: ResourceMap::default(),
        };

        dfs(self, init_state)
    }
}

const TIME_LIMIT: u32 = 32;

const N_BLUEPRINTS: usize = 3;

fn main() {
    let blueprints: Vec<Blueprint> = io::stdin()
        .lines()
        .take(N_BLUEPRINTS)
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    let geodes_product: u32 = blueprints
        .iter()
        .map(|b| b.max_geodes(TIME_LIMIT))
        .product();

    println!("{geodes_product}");
}
