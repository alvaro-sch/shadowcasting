use std::{
    //collections::HashSet,
    ops::{Index, IndexMut}
};

use crate::Line;

#[derive(Debug)]
pub struct Tilemap {
    pub width: usize,
    pub height: usize,
    tiles: Vec<u8>,
}

impl Tilemap {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height, tiles: vec![0; width * height] }
    }

    pub fn to_wallmap(&self, _scale: usize, _wallmap: &mut Vec<Line<i32>>) {
    }
}

impl Index<[usize; 2]> for Tilemap {
    type Output = u8;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        &self.tiles[index[0] + self.width * index[1]]
    }
}

impl IndexMut<[usize; 2]> for Tilemap {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        &mut self.tiles[index[0] + self.width * index[1]]
    }
}