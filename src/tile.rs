use std::cmp::Ordering::*;

use collapser::cell::{
    Collapseable::{self, *},
    Neighbors, Superposition,
};

use crate::rules::{direction_rules, relevant_rules, Direction, Rule, Rules};

use rand::prelude::*;

pub type TileID = u16;

#[derive(Clone)]
pub struct SuperTile(pub Vec<TileID>);
impl SuperTile {
    fn rm_impossible(&mut self, other: &[TileID], side: Direction, rules: &[Rule]) -> bool {
        let mut changed = false;
        let rules = direction_rules(rules, side.invert());
        let mut dir_possibilities: Vec<_> = other
            .iter()
            .map(|&origin| relevant_rules(origin, rules))
            .flatten()
            .map(|rule| rule.second)
            .collect();
        dir_possibilities.sort_unstable();
        dir_possibilities.dedup();
        let self_copy = self.0.clone();
        let (mut selfiter, mut otheriter) = (self_copy.iter().enumerate(), other.iter());
        let mut removed = 0;
        while let (Some((i, left)), Some(right)) = (selfiter.next(), otheriter.next()) {
            match left.cmp(right) {
                Equal => (),
                Less => {
                    changed = true;
                    self.0.remove(i - removed);
                    removed += 1;
                    while let Some((i, left)) = selfiter.next() {
                        if left < right {
                            self.0.remove(i - removed);
                            removed += 1;
                        } else {
                            break;
                        }
                    }
                }
                Greater => {
                    while let Some(right) = otheriter.next() {
                        if left <= right {
                            break;
                        }
                    }
                }
            }
        }
        changed
    }
}
impl Superposition for SuperTile {
    type Tile = TileID;
    type Rules = Rules;

    fn refine(
        &mut self,
        sides: Neighbors<Collapseable<&Self::Tile, &Self>>,
        rules: &Self::Rules,
    ) -> Result<Self::Tile, bool> {
        let changed = match sides.top {
            Collapsed(&0) => false,
            Collapsed(&t) => self.rm_impossible(&[t], Direction::Up, rules),
            Superimposed(other) => self.rm_impossible(&other.0, Direction::Up, rules),
        } || match sides.bottom {
            Collapsed(&0) => false,
            Collapsed(&t) => self.rm_impossible(&[t], Direction::Down, rules),
            Superimposed(other) => self.rm_impossible(&other.0, Direction::Down, rules),
        } || match sides.left {
            Collapsed(&0) => false,
            Collapsed(&t) => self.rm_impossible(&[t], Direction::Left, rules),
            Superimposed(other) => self.rm_impossible(&other.0, Direction::Left, rules),
        } || match sides.right {
            Collapsed(&0) => false,
            Collapsed(&t) => self.rm_impossible(&[t], Direction::Right, rules),
            Superimposed(other) => self.rm_impossible(&other.0, Direction::Right, rules),
        };
        match self.0.len() {
            0 => Ok(0),
            1 => Ok(self.0[0]),
            _ => Err(changed),
        }
    }

    fn reduce(&mut self) {
        let to_rm = thread_rng().gen_range(0..self.0.len());
        self.0.remove(to_rm);
    }
}
