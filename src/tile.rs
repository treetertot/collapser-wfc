use std::ops::Deref;
use std::slice::from_ref;

use collapser::cell::Working;

use crate::rules::Rules;

use rand::prelude::*;

pub type TileID = u16;

#[derive(Clone, Debug, PartialEq)]
pub struct SuperTile(pub Vec<TileID>);
impl Deref for SuperTile {
    type Target = [TileID];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Working for SuperTile {
    type Tile = TileID;
    type Rules = Rules;
    type Grabber = [(i32, i32); 8];

    const NEIGHBORS: Self::Grabber = [(-1, -1), (0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0)];

    fn new(rules: &Self::Rules) -> Self {
        let mut possible: Vec<_> = rules.iter()
            .map(|r| {
                r.center
            })
            .collect();
        possible.dedup();
        possible.shrink_to_fit();
        SuperTile(possible)
    }
    
    fn refine(&mut self, neighbors: &[Result<&Self::Tile, &Self>], rules: &Self::Rules) -> Result<Self::Tile, bool> {
        let mut iter = neighbors.iter().map(|r| match r {
            Ok(0) => &[],
            Ok(n) => from_ref(*n),
            Err(poss) => &poss.0
        });
        let neighbors = [
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
        ];
        let start_len = self.0.len();
        self.0.retain(|&n| {
            let sub_rules = rules.rules_for(n);
            sub_rules.iter()
                .any(|r| r.eval(&neighbors))
        });
        match self.0.len() {
            0 | 1 => Ok(self.0.pop().unwrap_or(0)),
            n => Err(n != start_len)
        }
    }
    fn force_collapse(&self) -> Self::Tile {
        let i = rand::thread_rng().gen_range(0..self.0.len());
        self.0.get(i).map(|&i| i).unwrap_or(0)
    }
}