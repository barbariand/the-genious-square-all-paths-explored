use crate::bitmap::BitMap64;
use std::fmt::Debug;
use std::fmt::Display;
pub struct PieceBoard {
    pub total: BitMap64,
    pieces: [BitMap64; 9],
}
impl PieceBoard {
    pub fn try_insert(&self, new: &BitMap64, i: usize) -> Option<Self> {
        (&self.total & new == BitMap64::new(0)).then(|| {
            let mut pieces = self.pieces.clone();
            pieces[i + 1] = new.clone();
            Self {
                total: &self.total | new,
                pieces,
            }
        })
    }
    #[inline(always)]
    pub fn insert(&self, new: BitMap64, total: BitMap64, i: usize) -> Self {
        let mut pieces = self.pieces.clone();
        pieces[i + 1] = new;
        Self { total, pieces }
    }
    pub fn new(first: BitMap64) -> Self {
        Self {
            total: first.clone(),
            pieces: [
                first,
                BitMap64::new(0),
                BitMap64::new(0),
                BitMap64::new(0),
                BitMap64::new(0),
                BitMap64::new(0),
                BitMap64::new(0),
                BitMap64::new(0),
                BitMap64::new(0),
            ],
        }
    }
}
impl Debug for PieceBoard {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use crate::pieces::Pieces::{
            OneByFour, OneByOne, OneByThree, OneByTwo, Shape6, Shape7, Shape8, Shape9, TwoByTwo,
        };
        for (i, piece) in [
            OneByOne, OneByTwo, OneByThree, TwoByTwo, Shape6, Shape7, Shape8, Shape9, OneByFour,
        ]
        .iter()
        .rev()
        .enumerate()
        {
            writeln!(f, "{:?}:\n{:?}", piece, self.pieces[i])?;
        }
        Ok(())
    }
}
impl Display for PieceBoard {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use crate::pieces::Pieces::{
            OneByFour, OneByOne, OneByThree, OneByTwo, Shape6, Shape7, Shape8, Shape9, TwoByTwo,
        };
        for (i, piece) in [
            OneByOne, OneByTwo, OneByThree, TwoByTwo, Shape6, Shape7, Shape8, Shape9, OneByFour,
        ]
        .iter()
        .rev()
        .enumerate()
        {
            writeln!(f, "{:?}:\n{:?}", piece, self.pieces[i])?;
        }
        Ok(())
    }
}
