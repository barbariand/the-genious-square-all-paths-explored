//! a program to find find all solutions given any particular board
#![warn(clippy::all, clippy::perf, clippy::pedantic)]
#![feature(test)]
#![feature(iter_map_windows)]
#![feature(portable_simd)]
#![feature(slice_as_chunks)]
#![feature(array_chunks)]
#![feature(iter_array_chunks)]
mod args;
mod bitmap;
mod dices;
mod generated_bitboards;
use bitmap::BitMap64;
use clap::Parser;
use dices::{Column, Dices, Row, DICE1, DICE2, DICE3, DICE4, DICE5, DICE6, DICE7};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use core::fmt::{Debug, Display};

struct DiceCombinationIterator<'a> {
    vecs: [&'a [(Row, Column)]; 7],
    indices: [usize; 7],
    done: bool,
}

impl<'a> DiceCombinationIterator<'a> {
    fn new() -> Self {
        DiceCombinationIterator {
            indices: [0; 7],
            vecs: [&DICE1, &DICE2, &DICE3, &DICE4, &DICE5, &DICE6, &DICE7],
            done: false,
        }
    }
}

impl<'a> Iterator for DiceCombinationIterator<'a> {
    type Item = Dices;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let result = [
            self.vecs[0][self.indices[0]],
            self.vecs[1][self.indices[1]],
            self.vecs[2][self.indices[2]],
            self.vecs[3][self.indices[3]],
            self.vecs[4][self.indices[4]],
            self.vecs[5][self.indices[5]],
            self.vecs[6][self.indices[6]],
        ];

        // Increment the indices
        for i in (0..7).rev() {
            if self.indices[i] < self.vecs[i].len() - 1 {
                self.indices[i] += 1;
                break;
            } else {
                self.indices[i] = 0;
                if i == 0 {
                    self.done = true;
                }
            }
        }

        Some(Dices::new(result))
    }
}

struct BoardIterator {
    vec: [usize; 7],
}

impl BoardIterator {
    fn new() -> Self {
        Self {
            vec: [0, 1, 2, 3, 4, 5, 6],
        }
    }
}
impl Iterator for BoardIterator {
    type Item = Board;
    fn next(&mut self) -> Option<Self::Item> {
        // incrementing where possible
        let moved = self.vec.iter_mut().rev().enumerate().find_map(|(i, v)| {
            (*v < 35 - i).then(|| {
                *v += 1;
                i
            })
        })?;

        // now we may have things that have reatched the end and we need to move them
        for (i, revi) in (0..moved).rev().enumerate() {
            self.vec[6 - revi] = self.vec[6 - moved] + (i + 1);
        }

        Some(Board::from_pos(&self.vec))
    }
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Board;

    use super::dices::*;
    use test::Bencher;

    #[bench]
    fn find_all_solutions_to_all_dice_rolls(b: &mut Bencher) {
        b.iter(|| {
            Board::new(get_dices()).solve();
        })
    }
}
const INVALID: BitMap64 = BitMap64::new(18_446_720_803_221_662_421);
fn main() {
    /*use indicatif::ProgressIterator;
    println!(
        "{}",
        BoardIterator::new()
            .filter(|v| v.starter_board & invalid == BitMap64::new(0))
            .count()
    );
    todo!();
    let iterator = BoardIterator::new();

    let mut all_solutions: usize = iterator
        .par_bridge()
        .filter(|v| v.starter_board & invalid == BitMap64::new(0))
        .map(|board| {
            let dieces = board.starter_board.clone();
            let len = board.solve().len();
            println!("dieces:{},solutions{}", dieces.get_copied_inner(), len);
            len
        })
        .sum();*/

    // Process the board as needed
    /*
    println!("all solutions:{:?}", all_solutions);
    */
    let parse = args::CustomArgs::parse();
    let args = parse;

    let mut starter_board = BitMap64::new(0);
    for (r, c) in args.dieces.0 {
        starter_board.set_bit(r as u64 * 8 + c as u64);
    }
    let board = Board::new(args.dieces);
    let solutions = board.solve();

    println!(
        "starterboard:\n{:?}\n{}",
        starter_board,
        solutions
            .first()
            .map_or_else(|| "None".to_owned(), ToString::to_string)
    );

    println!("amount of solutions fund: {}", solutions.len());
    std::fs::write("./solutions", format!("{solutions:?}")).expect("cant write to file");
}
#[derive(Debug, Clone, Copy)]
struct Board {
    starter_board: BitMap64,
}

impl Board {
    fn from_pos(vec: &[usize; 7]) -> Self {
        let mut starter_board = BitMap64::new(0);
        for i in vec {
            starter_board.set_bit((i / 6 * 8 + i % 6) as u64);
        }
        Self { starter_board }
    }
    fn new(dices: Dices) -> Self {
        let mut starter_board = BitMap64::new(0);
        for (r, c) in dices.0 {
            starter_board.set_bit(r as u64 * 8 + c as u64);
        }
        Self { starter_board }
    }
    fn solve(self) -> Vec<PieceBoard> {
        use std::simd::*;

        let mut possible: Vec<Vec<&BitMap64>> = pieces::get_possible(self.starter_board);
        let pre_candidates: Vec<&BitMap64> = possible
            .pop()
            .expect("it should allways have 9 in the thingymagig");
        //println!("allocating {}u64s", pre_candidates.len() * 10);
        let candidates: Vec<_> = pre_candidates
            .par_iter()
            .map(|v| PieceBoard::new(**v))
            .collect();
        possible
            .iter()
            .rev()
            .enumerate()
            .fold(candidates, |acc, (i, piece_positions)| {
                piece_positions
                    .par_iter()
                    .flat_map_iter(|v: &&BitMap64| {
                        let compare = u64x64::splat(v.get_copied_inner());
                        let iter = acc.array_chunks();

                        let rem = iter.remainder();
                        iter.flat_map(move |val: &[PieceBoard; 64]| {
                            let vec = val
                                .iter()
                                .map(|v| v.total.get_copied_inner())
                                .collect::<Vec<_>>();
                            let arr = u64x64::from_slice(&(*vec));
                            let anded = (arr & compare).to_array();
                            let ored = (arr | compare).to_array();
                            anded.into_iter().zip(ored.into_iter()).zip(val).filter_map(
                                move |((comp, new), val)| {
                                    (comp == 0).then(|| val.insert(**v, BitMap64::new(new), i))
                                },
                            )
                        })
                        .chain(rem.iter().filter_map(|val| val.try_insert(v, i)))
                    })
                    .collect()
            })
    }
}

/* fn accumilative_solve(
    acc: impl ParallelIterator<Item = PieceBoard>,
    (i, piece_positions): (usize, &Vec<&BitMap64>),
) -> impl ParallelIterator<Item = PieceBoard> {
    piece_positions
        .par_iter()
        .flat_map_iter(|v| acc.filter_map(|val| val.try_insert(v, i)))
} */

struct PieceBoard {
    total: BitMap64,
    pieces: [BitMap64; 9],
}
impl PieceBoard {
    fn try_insert(&self, new: &BitMap64, i: usize) -> Option<Self> {
        if !(self.total & *new == BitMap64::new(0)) {
            return None;
        }
        let mut pieces = self.pieces;
        pieces[i + 1] = *new;
        Some(Self {
            total: self.total | *new,
            pieces,
        })
    }
    fn insert(&self, new: BitMap64, total: BitMap64, i: usize) -> Self {
        let mut pieces = self.pieces;
        pieces[i + 1] = new;
        Self { total, pieces }
    }
    fn new(first: BitMap64) -> Self {
        Self {
            total: first,
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
        use pieces::Pieces::{
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
        use pieces::Pieces::{
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
mod pieces {

    use crate::bitmap::BitMap64;
    use crate::generated_bitboards::*;
    use rayon::iter::IntoParallelRefIterator;
    use rayon::iter::ParallelIterator;
    /// gets all possible
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
}
