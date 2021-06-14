#[cfg(test)]
mod tests {
    #[test]
    fn four_square() {
        // could just be an array but rust analyzer gets mad at me
        let patterns = vec![
            [
                4, 2, 4,
                2, 1, 2,
                4, 2, 4
            ],
            [
                3, 4, 3,
                1, 2, 1,
                3, 4, 3
            ],
            [
                2, 1, 2,
                4, 3, 4,
                2, 1, 2
            ],
            [
                1, 2, 1,
                3, 4, 3,
                1, 2, 1
            ],
        ];
        let rules = crate::rules::Rules::new(patterns);
        let mut world = crate::World::new(rules, [0, 0]..[2, 2]);
        for x in 0..2 {
            for y in 0..2 {
                world.collapse(x, y);
            }
        }
        let result = [world.read(0, 0), world.read(1, 0), world.read(0, 1), world.read(1, 1)];
        assert!(result[0] != result[1] && result[1] != result[2] && result[2] != result[3]);
    }
}

pub mod rules;
pub mod tile;

pub type World = collapser::world::World<tile::SuperTile>;
