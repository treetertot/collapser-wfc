use crate::tile::TileID;
use std::ops::Deref;

/// A possible configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rule {
    pub center: TileID,
    pub around: [TileID; 8],
}
impl Rule {
    pub fn new(pattern: [TileID; 9]) -> Self {
        Rule {
            center: pattern[4],
            around: [pattern[6], pattern[7], pattern[8], pattern[5], pattern[2], pattern[1], pattern[0], pattern[3]]
        }
    }
    pub fn eval(&self, neighbors: &[&[TileID]; 8]) -> bool {
        self
            .around
            .iter()
            .zip(neighbors)
            .filter(|(_, n)| n.len() > 0)
            .all(|(target, list)| list.binary_search(target).is_ok())
    }
}
#[test]
fn construct() {
    let pattern = [
        1, 2, 3,
        4, 5, 6,
        7, 8, 9
    ];
    assert_eq!(Rule::new(pattern), Rule{center: 5, around: [7, 8, 9, 6, 3, 2, 1, 4]})
}
#[test]
fn matching() {
    let pattern = [
        1, 2, 3,
        4, 5, 6,
        7, 8, 9
    ];
    let rule = Rule::new(pattern);
    let ns = [&[7][..], &[8], &[9], &[6], &[3], &[2], &[1], &[4]];
    assert!(rule.eval(&ns));
    let ns = [&[7][..], &[8], &[9], &[6], &[3], &[2], &[1], &[5]];
    assert!(!rule.eval(&ns));
}

#[derive(Debug, Clone)]
pub struct Rules(Vec<Rule>);
impl Rules {
    pub fn new<I: IntoIterator<Item=[TileID; 9]>>(patterns: I) -> Self {
        let mut list: Vec<_> = patterns.into_iter().map(|pattern| Rule::new(pattern)).collect();
        list.sort();
        list.dedup();
        list.shrink_to_fit();
        Rules(list)
    }
    pub fn rules_for(&self, center: TileID) -> &[Rule] {
        let start = match self.0.binary_search(&Rule{
            center,
            around: [TileID::MIN; 8]
        }) {
            Ok(i) => i,
            Err(i) => i
        };
        match self.0.binary_search(&Rule{
            center,
            around: [TileID::MAX; 8]
        }) {
            Ok(i) => &self.0[start..=i],
            Err(i) => &self.0[start..i]
        }
    }
}
impl Deref for Rules {
    type Target = [Rule];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}