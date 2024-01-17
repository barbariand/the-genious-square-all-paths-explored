use crate::bitmap;
use crate::bitmap::BitMap64;
use crate::generated_bitboards::*;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
/// gets all possible given the starter bitmap by using the generated code
#[inline]
pub fn get_possible(bitmap: BitMap64) -> ([Vec<&'static BitMap64>; 8], Vec<&'static BitMap64>) {
    (
        [
            ONEBYONE
                .iter()
                .filter(|v| (*v & &bitmap) == bitmap::ZERO)
                .collect(),
            ONEBYTWO
                .iter()
                .filter(|v| (*v & &bitmap) == bitmap::ZERO)
                .collect(),
            ONEBYTHREE
                .iter()
                .filter(|v| (*v & &bitmap) == bitmap::ZERO)
                .collect(),
            TWOBYTWO
                .iter()
                .filter(|v| (*v & &bitmap) == bitmap::ZERO)
                .collect(),
            SHAPE6
                .iter()
                .filter(|v| (*v & &bitmap) == bitmap::ZERO)
                .collect(),
            SHAPE7
                .iter()
                .filter(|v| (*v & &bitmap) == bitmap::ZERO)
                .collect(),
            SHAPE8
                .iter()
                .filter(|v| (*v & &bitmap) == bitmap::ZERO)
                .collect(),
            SHAPE9
                .iter()
                .filter(|v| (*v & &bitmap) == bitmap::ZERO)
                .collect(),
        ],
        ONEBYFOUR
            .iter()
            .filter(|v| (*v & &bitmap) == bitmap::ZERO)
            .collect(),
    )
}
#[cfg(test)]
mod test {
    use std::hint::black_box;

    use crate::{
        bitmap::{self, BitMap64},
        generated_bitboards::SHAPE6,
    };

    use super::{
        get_possible, ONEBYFOUR, ONEBYONE, ONEBYTHREE, ONEBYTWO, SHAPE7, SHAPE8, SHAPE9, TWOBYTWO,
    };
    extern crate test;
    use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
    use test::Bencher;
    #[bench]
    fn testing_random_get_posible(b: &mut Bencher) {
        b.iter(|| get_possible(BitMap64::new(rand::random())))
    }
    #[bench]
    fn testing_random_get_posible_new(b: &mut Bencher) {
        b.iter(|| get_possible_new(BitMap64::new(rand::random())))
    }
    #[bench]
    fn getting_all(b: &mut Bencher) {
        b.iter(|| get_possible(BitMap64::new(0)))
    }
    #[bench]
    fn getting_all_with_new(b: &mut Bencher) {
        b.iter(|| get_possible_new(BitMap64::new(0)))
    }
    #[bench]
    fn testing_random_get_posible_newer(b: &mut Bencher) {
        b.iter(|| get_possible_newer(BitMap64::new(rand::random())))
    }
    #[bench]
    fn getting_all_newer(b: &mut Bencher) {
        b.iter(|| get_possible_newer(BitMap64::new(0)))
    }
    #[inline]
    #[allow(dead_code)]
    pub fn get_possible_new(bitmap: BitMap64) -> Vec<Vec<&'static BitMap64>> {
        [
            ONEBYONE.as_ref(),
            ONEBYTWO.as_ref(),
            ONEBYTHREE.as_ref(),
            TWOBYTWO.as_ref(),
            SHAPE6.as_ref(),
            SHAPE7.as_ref(),
            SHAPE8.as_ref(),
            SHAPE9.as_ref(),
            ONEBYFOUR.as_ref(),
        ]
        .into_iter()
        .map(|arr| {
            arr.into_iter()
                .filter(|v| (*v & &bitmap) == bitmap::ZERO)
                .collect()
        })
        .collect()
    }
    #[inline]
    #[allow(dead_code)]
    pub fn get_possible_newer(bitmap: BitMap64) -> [Vec<&'static BitMap64>; 9] {
        [
            ONEBYONE
                .iter()
                .filter(|v| (*v & &bitmap) == bitmap::ZERO)
                .collect(),
            ONEBYTWO
                .iter()
                .filter(|v| (*v & &bitmap) == bitmap::ZERO)
                .collect(),
            ONEBYTHREE
                .iter()
                .filter(|v| (*v & &bitmap) == bitmap::ZERO)
                .collect(),
            TWOBYTWO
                .iter()
                .filter(|v| (*v & &bitmap) == bitmap::ZERO)
                .collect(),
            SHAPE6
                .iter()
                .filter(|v| (*v & &bitmap) == bitmap::ZERO)
                .collect(),
            SHAPE7
                .iter()
                .filter(|v| (*v & &bitmap) == bitmap::ZERO)
                .collect(),
            SHAPE8
                .iter()
                .filter(|v| (*v & &bitmap) == bitmap::ZERO)
                .collect(),
            SHAPE9
                .iter()
                .filter(|v| (*v & &bitmap) == bitmap::ZERO)
                .collect(),
            ONEBYFOUR
                .iter()
                .filter(|v| (*v & &bitmap) == bitmap::ZERO)
                .collect(),
        ]
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
