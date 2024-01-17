use crate::bitmap::BitMap64;
use crate::generated_bitboards::*;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
/// gets all possible given the starter bitmap by using the generated code
#[inline]
pub fn get_possible(bitmap: BitMap64) -> Vec<Vec<&'static BitMap64>> {
    [
        (0_usize, Pieces::OneByOne),
        (1, Pieces::OneByTwo),
        (2, Pieces::OneByThree),
        (4, Pieces::TwoByTwo),
        (5, Pieces::Shape6),
        (6, Pieces::Shape7),
        (7, Pieces::Shape8),
        (8, Pieces::Shape9),
        (3, Pieces::OneByFour),
    ]
    .par_iter()
    .map(|(_index, piece)| {
        let arr = piece.get_array();
        arr.iter()
            .filter(|v| (**v & bitmap).get_copied_inner() == 0)
            .collect()
    })
    .collect()
}
#[cfg(test)]
mod test {
    use std::hint::black_box;

    use crate::bitmap::BitMap64;

    use super::{get_possible, Pieces};
    extern crate test;
    use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
    use test::Bencher;
    #[bench]
    fn testing_random_get_posible(b: &mut Bencher) {
        b.iter(|| black_box(get_possible(BitMap64::new(rand::random()))))
    }
    #[bench]
    fn getting_all(b: &mut Bencher) {
        b.iter(|| black_box(get_possible(BitMap64::new(0))))
    }
    #[bench]
    fn getting_all_with_new(b: &mut Bencher) {
        b.iter(|| black_box(get_possible(BitMap64::new(0))))
    }
    #[inline]
    pub fn get_possible_new(bitmap: BitMap64) -> Vec<Vec<&'static BitMap64>> {
        [
            (0_usize, Pieces::OneByOne),
            (1, Pieces::OneByTwo),
            (2, Pieces::OneByThree),
            (4, Pieces::TwoByTwo),
            (5, Pieces::Shape6),
            (6, Pieces::Shape7),
            (7, Pieces::Shape8),
            (8, Pieces::Shape9),
            (3, Pieces::OneByFour),
        ]
        .par_iter()
        .map(|(_index, piece)| {
            let arr = piece.get_array();
            arr.into_iter()
                .filter(|v| (**v & bitmap) == BitMap64::new(0))
                .collect()
        })
        .collect()
    }
}

#[derive(Debug)]
pub enum Pieces {
    OneByOne,
    OneByTwo,
    OneByThree,
    OneByFour,
    TwoByTwo,
    Shape6,
    Shape7,
    Shape8,
    Shape9,
}

impl Pieces {
    fn get_array(&self) -> &'static [BitMap64] {
        match self {
            Self::OneByOne => return ONEBYONE.as_ref(),
            Self::OneByTwo => return ONEBYTWO.as_ref(),
            Self::OneByThree => return ONEBYTHREE.as_ref(),
            Self::OneByFour => return ONEBYFOUR.as_ref(),
            Self::TwoByTwo => return TWOBYTWO.as_ref(),
            Self::Shape6 => return SHAPE6.as_ref(),
            Self::Shape7 => return SHAPE7.as_ref(),
            Self::Shape8 => return SHAPE8.as_ref(),
            Self::Shape9 => return SHAPE9.as_ref(),
        }
    }
}
