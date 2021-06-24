use std::iter::once;
use std::ops::Deref;
use std::slice::from_ref;

use collapser::cell::Working;

use crate::rules::Rules;

use rand::prelude::*;

pub type TileID = u16;

#[derive(Clone, Debug, PartialEq)]
pub struct SuperTile{
    ids: Vec<TileID>,
    scores: Weighted
}
impl Deref for SuperTile {
    type Target = [TileID];

    fn deref(&self) -> &Self::Target {
        &self.ids
    }
}
impl Working for SuperTile {
    type Tile = TileID;
    type Rules = Rules;
    type Grabber = [(i32, i32); 8];

    const NEIGHBORS: Self::Grabber = [(-1, -1), (0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0)];

    fn new(rules: &Self::Rules) -> Self {
        let iter = rules.iter()
            .map(|r| {
                r.center
            });
        let (ids, scores) = Weighted::count(iter);

        SuperTile {
            scores,
            ids
        }
    }
    
    fn refine(&mut self, neighbors: &[Result<&Self::Tile, &Self>], rules: &Self::Rules) -> Result<Self::Tile, bool> {
        let mut iter = neighbors.iter().map(|r| match r {
            Ok(0) => &[],
            Ok(n) => from_ref(*n),
            Err(poss) => &poss.ids
        });
        let neighbors = [
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap()
        ];
        let start_len = self.ids.len();
        self.ids.retain(|&n| {
            let sub_rules = rules.rules_for(n);
            sub_rules.iter()
                .any(|r| r.eval(&neighbors))
        });
        for (id, w_out) in self.ids.iter().zip(&mut self.scores.0) {
            let relevant = rules.rules_for(*id);
            *w_out = relevant.iter().map(|r| r.score_eval(&neighbors)).sum();
        }
        let mut rmed = 0;
        for i in 0..self.scores.0.len() {
            if self.ids.len() == 0 {
                break;
            }
            let i = i - rmed;
            if self.scores.0[i] == 0 {
                self.scores.0.remove(i);
                self.ids.remove(i);
                rmed += 1;
            }
        }
        match self.ids.len() {
            0 | 1 => Ok(self.ids.pop().unwrap_or(0)),
            n => Err(n != start_len)
        }
    }
    fn force_collapse(&self) -> Self::Tile {
        self.scores.choose_rand()
            .map(|i| self.ids[i])
            .unwrap_or(0)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Weighted(Vec<i32>);
impl Weighted {
    fn count<I: IntoIterator<Item=TileID>>(iter: I) -> (Vec<u16>, Self) {
        let mut last = TileID::MAX;
        let mut l_idx = 0;
        let mut iter = iter.into_iter()
            .chain(once(TileID::MAX))
            .enumerate()
            .filter_map(|(i, n)| match last != n {
                true => {
                    let li = l_idx;
                    l_idx = i;
                    let l = last;
                    last = n;
                    Some((l, (i - li) as i32))
                },
                false => None
            });
        iter.next();
        let (ids, weights) = iter
            .unzip();
        (ids, Weighted(weights))
    }
    fn choose(&self, mut n: i32) -> Option<usize> {
        for (i, x) in self.0.iter().enumerate() {
            n -= x;
            if n <= 0 {
                return Some(i)
            }
        }
        None
    }
    fn sum(&self) -> i32 {
        self.0.iter().sum()
    }
    fn choose_rand(&self) -> Option<usize> {
        let n = thread_rng().gen_range(1..=self.sum());
        self.choose(n)
    }
}
#[test]
fn count_test() {
    let items = vec![1, 1, 1, 2, 2, 3, 4];
    let (ids, weights) = Weighted::count(items.clone());
    let mut new_items = Vec::new();
    for (id, weight) in ids.into_iter().zip(weights.0) {
        for _ in 0..weight {
            new_items.push(id)
        }
    }
    assert_eq!(items, new_items);
}
#[test]
fn wbasetest() {
    let w = Weighted(vec![0, 1, 1, 0]);
    assert_eq!(2, w.sum());
    let choice = w.choose_rand();
    println!("{:?}", choice);
    assert_ne!(choice, None);
    assert!(choice == Some(1) || choice == Some(2));

}
#[test]
fn weightingtest() {
    let w = Weighted(vec![1, 3]);
    let mut res = [0u32; 2];
    for _i in 0..100000 {
        let choice = w.choose_rand();
        res[choice.expect("this does not work at all")] += 1;
    }
    assert!(res[1] > res[0] * 2);
    assert!(res[1] < res[0] * 4);
}