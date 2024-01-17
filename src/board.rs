use super::dices::Dices;
use crate::pieceboard::PieceBoard;
use crate::BitMap64;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
#[derive(Debug, Clone, Copy)]
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
        use std::simd::*;
        // we get the possible pieces here that are not in the starterboard
        let mut possible: Vec<Vec<&BitMap64>> = crate::pieces::get_possible(self.starter_board);
        //we start by using the first one beacuse it is garanted to not be in the bitbaord
        let pre_candidates: Vec<&BitMap64> = possible
            .pop()
            .expect("it should allways have 9 vecs in the thingy");
        //make them to pieceboards holadin all pieces
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
