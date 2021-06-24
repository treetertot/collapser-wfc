use crate::tile::TileID;
use std::ops::Deref;

/// A possible configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rule {
    pub center: TileID,
    pub around: [TileID; 8],
    pub score: i32,
}
impl Rule {
    pub fn new(pattern: [TileID; 9], score: i32) -> Self {
        Rule {
            center: pattern[4],
            around: [pattern[6], pattern[7], pattern[8], pattern[5], pattern[2], pattern[1], pattern[0], pattern[3]],
            score
        }
    }
}
#[test]
fn construct() {
    let pattern = [
        1, 2, 3,
        4, 5, 6,
        7, 8, 9
    ];
    assert_eq!(Rule::new(pattern, 1), Rule{center: 5, around: [7, 8, 9, 6, 3, 2, 1, 4], score: 1})
}

#[derive(Debug, Clone)]
pub struct Rules(Vec<Rule>);
impl Rules {
    pub fn new<I: IntoIterator<Item=([TileID; 9], i32)>>(patterns: I) -> Self {
        let mut list: Vec<_> = patterns.into_iter().map(|(pattern, score)| Rule::new(pattern, score)).collect();
        list.sort();
        list.dedup();
        list.shrink_to_fit();
        Rules(list)
    }
    pub fn rules_for(&self, center: TileID) -> &[Rule] {
        let start = match self.0.binary_search(&Rule{
            center,
            around: [TileID::MIN; 8],
            score: 0
        }) {
            Ok(i) => i,
            Err(i) => i
        };
        match self.0.binary_search(&Rule{
            center,
            around: [TileID::MAX; 8],
            score: i32::MAX
        }) {
            Ok(i) => &self.0[start..=i],
            Err(i) => &self.0[start..i]
        }
    }
    pub fn score(&self, centers: &[TileID], score_outs: &mut [i32], neighbors: &[&[TileID]; 8]) {
        for (center, score_out) in centers.iter().zip(score_outs) {
            let relevant = self.rules_for(*center);
            *score_out = score_rules(relevant, neighbors);
        }
    }
}
impl Deref for Rules {
    type Target = [Rule];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn score_rules(rules: &[Rule], neighbors: &[&[TileID]; 8]) -> i32 {
    let mut indexes = [0usize; 8];
    let mut sum = 0;
    for rule in rules {
        let mut matched = true;
        for ((pattern, options), idx) in rule.around.iter().zip(neighbors).zip(&mut indexes) {
            match options.get(*idx) {
                Some(n) if n == pattern => (),
                // item at index is too high
                Some(n) if n > pattern => {
                    let searchable = &options[..*idx];
                    match searchable.binary_search(pattern) {
                        Ok(i) => *idx = i,
                        Err(i) => {
                            *idx = i;
                            matched = false;
                            break
                        }
                    }
                },
                // item at index is too low
                Some(n) => {
                    let mut n = *n;
                    while n < *pattern {
                        *idx += 1;
                        match options.get(*idx) {
                            Some(&new_n) => n = new_n,
                            None => break
                        }
                    }
                    if n != *pattern {
                        matched = false;
                        break
                    }
                },
                None => ()
            }
        }
        if matched {
            sum += rule.score;
        }
    }
    sum
}

#[test]
fn score_test() {
    // I'd just use an array but rust analyzer gets mad
    let patterns = vec![
        (
            [
                1, 1, 1,
                1, 1, 1,
                1, 1, 1
            ],
            1
        ),
        (
            [
                1, 1, 1,
                1, 1, 1,
                1, 1, 2
            ],
            2
        ),
        (
            [
                1, 1, 2,
                1, 1, 1,
                1, 1, 1
            ],
            3
        ),
    ];
    let rules = Rules::new(patterns);
    let pattern = [
        &[1u16][..], &[1], &[1, 2],
        &[1], &[1], &[1],
        &[1], &[1], &[1]
    ];
    let neighbors = [pattern[6], pattern[7], pattern[8], pattern[5], pattern[2], pattern[1], pattern[0], pattern[3]];
    assert_eq!(score_rules(&rules, &neighbors), 4)
}

