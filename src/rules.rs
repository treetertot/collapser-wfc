use crate::tile::{SuperTile, TileID};
use std::{
    iter,
    ops::{Deref, Range},
};

use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    pub fn invert(&self) -> Self {
        use Direction::*;
        match self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        }
    }
}

/// A possible configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rule {
    /// The direction that second is from origin
    pub direction: Direction,
    pub origin: TileID,
    pub second: TileID,
}

#[derive(Debug, Deserialize)]
pub struct TileCfg {
    id: TileID,
    above: Vec<TileID>,
    below: Vec<TileID>,
    left: Vec<TileID>,
    right: Vec<TileID>,
}

pub struct Rules(Vec<Rule>);
impl Rules {
    pub fn new<L: IntoIterator<Item = TileCfg>>(rules: L) -> Self {
        let mut inside: Vec<_> = rules
            .into_iter()
            .map(|tile| {
                let origin = tile.id;
                tile.above
                    .into_iter()
                    .map(move |second| {
                        iter::once(Rule {
                            origin,
                            second,
                            direction: Direction::Up,
                        })
                        .chain(iter::once(Rule {
                            origin: second,
                            second: origin,
                            direction: Direction::Down,
                        }))
                    })
                    .flatten()
                    .chain(
                        tile.below
                            .into_iter()
                            .map(move |second| {
                                iter::once(Rule {
                                    origin,
                                    second,
                                    direction: Direction::Down,
                                })
                                .chain(iter::once(Rule {
                                    origin: second,
                                    second: origin,
                                    direction: Direction::Up,
                                }))
                            })
                            .flatten(),
                    )
                    .chain(
                        tile.left
                            .into_iter()
                            .map(move |second| {
                                iter::once(Rule {
                                    origin,
                                    second,
                                    direction: Direction::Left,
                                })
                                .chain(iter::once(Rule {
                                    origin: second,
                                    second: origin,
                                    direction: Direction::Right,
                                }))
                            })
                            .flatten(),
                    )
                    .chain(
                        tile.right
                            .into_iter()
                            .map(move |second| {
                                iter::once(Rule {
                                    origin,
                                    second,
                                    direction: Direction::Right,
                                })
                                .chain(iter::once(Rule {
                                    origin: second,
                                    second: origin,
                                    direction: Direction::Left,
                                }))
                            })
                            .flatten(),
                    )
            })
            .flatten()
            .map(|mut rule| {
                if rule.origin == 0 {
                    rule.origin = 1;
                }
                if rule.second == 0 {
                    rule.second = 0;
                }
                rule
            })
            .collect();
        inside.sort_unstable();
        inside.dedup();
        inside.shrink_to_fit();
        Rules(inside)
    }
    /// Get a superposition of all posible tiles
    pub fn complete_superposition(&self) -> SuperTile {
        let mut inside: Vec<_> = self.0.iter().map(|r| r.origin).collect();
        inside.sort();
        inside.dedup();
        inside.shrink_to_fit();
        SuperTile(inside)
    }
}
impl Deref for Rules {
    type Target = [Rule];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn rules_between(rules: &[Rule], range: Range<Rule>) -> &[Rule] {
    let bottom = match rules.binary_search(&range.start) {
        Ok(n) => n,
        Err(n) => n,
    };
    match rules.binary_search(&range.end) {
        Ok(n) => &rules[bottom..=n],
        Err(n) => &rules[bottom..n],
    }
}

pub fn relevant_rules(origin: TileID, rules: &[Rule]) -> &[Rule] {
    let direction = match rules.first() {
        Some(r) => r.direction,
        None => return &[],
    };
    rules_between(
        rules,
        Rule {
            direction,
            origin,
            second: 0,
        }..Rule {
            direction,
            origin,
            second: TileID::MAX,
        },
    )
}
pub fn direction_rules(rules: &[Rule], direction: Direction) -> &[Rule] {
    rules_between(
        rules,
        Rule {
            direction,
            origin: 0,
            second: 0,
        }..Rule {
            direction,
            origin: TileID::MAX,
            second: TileID::MAX,
        },
    )
}
