#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod rules;
pub mod tile;

pub type World = collapser::world::TileWorld<tile::SuperTile>;
