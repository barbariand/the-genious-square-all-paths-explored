use super::dices::Dices;
use crate::pieceboard::PieceBoard;
use crate::BitMap64;
use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelIterator;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use rayon::prelude::ParallelBridge;
use std::simd::*;
#[derive(Debug, Clone)]
pub struct Board {
    starter_board: BitMap64,
}
impl Board {
    pub fn from_pos(vec: &[usize; 7]) -> Self {
        let mut starter_board = BitMap64::new(0);
        for i in vec {
            starter_board.set_bit((i / 6 * 8 + i % 6) as u64);
        }
        Self { starter_board }
    }
    pub fn new(dices: Dices) -> Self {
        let mut starter_board = BitMap64::new(0);
        for (r, c) in dices.0 {
            starter_board.set_bit(r as u64 * 8 + c as u64);
        }
        Self { starter_board }
    }
    pub fn solve(self) -> Vec<PieceBoard> {
        // we get the possible pieces here that are not in the starterboard
        let (possible, pre_candidates) = crate::pieces::get_possible(self.starter_board);
        let ZEROSIMD = u64x64::splat(0);
        //make them to pieceboards holadin all pieces
        let candidates: Vec<_> = pre_candidates
            .into_par_iter()
            .map(|v| PieceBoard::new(v.clone()))
            .collect();
        possible
            .into_iter()
            .fold(candidates, |acc, (i, piece_positions)| {
                piece_positions
                    .into_par_iter()
                    .flat_map(|v| {
                        let func = move |((comp, new), val): ((u64, u64), &PieceBoard)| {
                            (comp == 0).then(|| val.insert(v.clone(), BitMap64::new(new), i))
                        };
                        let compare = u64x64::splat(v.get_copied_inner());
                        let iter = acc.array_chunks();
                        let rem = iter.remainder();
                        iter.into_iter()
                            .par_bridge()
                            .filter_map(move |val: &[PieceBoard; 64]| {
                                let vec = val
                                    .iter()
                                    .map(|v| v.total.get_copied_inner())
                                    .collect::<Vec<_>>();
                                let arr = u64x64::from_slice(&(*vec));
                                let anded = arr & compare;
                                let ored = arr | compare;
                                if anded == ZEROSIMD {
                                    return None;
                                }
                                Some(
                                    anded
                                        .to_array()
                                        .into_par_iter()
                                        .zip(ored.to_array().into_par_iter())
                                        .zip(val.par_iter())
                                        .filter_map(func),
                                )
                            })
                            .flatten()
                            .chain(
                                rem.par_iter()
                                    .filter_map(|val: &PieceBoard| val.try_insert(v, i)),
                            )
                    })
                    .collect::<Vec<_>>()
            })
    }
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::Board;

    use crate::dices::get_dices;
    use test::Bencher;

    #[bench]
    fn finding_solution_to_random_board(b: &mut Bencher) {
        b.iter(|| {
            Board::new(get_dices()).solve();
        })
    }
    #[bench]
    fn finding_solution_to_bigest_board(b: &mut Bencher) {
        use crate::dices::{
            Column::{Five, Four, One, Six, Tree, Two},
            Dices,
            Row::{A, B},
        };
        b.iter(|| {
            Board::new(Dices::new([
                (A, One),
                (A, Two),
                (A, Tree),
                (A, Four),
                (A, Five),
                (A, Six),
                (B, One),
            ]))
            .solve();
        })
    }
}
