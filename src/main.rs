#![feature(iter_array_chunks)]
#![feature(test)]
#![feature(iter_map_windows)]
mod bitmap;

mod app;
mod generated_bitboards;
use bitmap::BitMap64;
use clap::Parser;
use dices::DICE1;
use dices::DICE2;
use dices::DICE3;
use dices::DICE4;
use dices::DICE5;
use dices::DICE6;
use dices::DICE7;
use indicatif::ParallelProgressIterator;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelIterator;
use rayon::prelude::ParallelBridge;
use std::fmt::Debug;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Clone, Copy, Debug)]
enum Row {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
}
#[derive(Clone, Copy, Debug)]
enum Column {
    One = 0,
    Two = 1,
    Tree = 2,
    Four = 3,
    Five = 4,
    Six = 5,
}

mod dices {
    use crate::{Column, Dices, Row};
    use rand::{self, Rng};

    use super::{Column::*, Row::*};

    pub const DICE1: [(Row, Column); 2] = [(F, One), (A, Six)];
    pub const DICE2: [(Row, Column); 6] = [
        (E, Four),
        (E, Five),
        (E, Six),
        (D, Five),
        (F, Four),
        (F, Five),
    ];
    pub const DICE3: [(Row, Column); 6] =
        [(A, One), (C, One), (D, One), (D, Two), (E, Two), (F, Tree)];
    pub const DICE4: [(Row, Column); 6] = [
        (A, Four),
        (B, Five),
        (C, Five),
        (C, Six),
        (F, Six),
        (D, Six),
    ];

    pub const DICE5: [(Row, Column); 6] =
        [(A, Tree), (B, One), (B, Two), (A, Two), (B, Tree), (C, Two)];
    pub const DICE6: [(Row, Column); 6] = [
        (B, Four),
        (C, Tree),
        (C, Four),
        (D, Tree),
        (D, Four),
        (E, Tree),
    ];
    pub const DICE7: [(Row, Column); 4] = [(A, Five), (F, Two), (B, Six), (E, One)];
    pub fn get_dices() -> Dices {
        let mut rng = rand::thread_rng();
        let dice1 = DICE1[rng.gen_range(0..DICE1.len())];
        let dice2 = DICE2[rng.gen_range(0..DICE2.len())];
        let dice3 = DICE3[rng.gen_range(0..DICE3.len())];
        let dice4 = DICE4[rng.gen_range(0..DICE4.len())];
        let dice5 = DICE5[rng.gen_range(0..DICE5.len())];
        let dice6 = DICE6[rng.gen_range(0..DICE6.len())];
        let dice7 = DICE7[rng.gen_range(0..DICE7.len())];
        Dices([dice1, dice2, dice3, dice4, dice5, dice6, dice7])
    }
}
#[derive(Clone, Copy, Debug)]
struct Dices([(Row, Column); 7]);
impl FromStr for Dices {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s
            .chars()
            .enumerate()
            .array_chunks()
            .map(|[(i, a), (j, n), d]| {
                println!("a:{:?},b:{:?},c:{:?}", (i, a), (j, n), d);
                Ok((
                    match a {
                        'A' => Row::A,
                        'B' => Row::B,
                        'C' => Row::C,
                        'D' => Row::D,
                        'E' => Row::E,
                        'F' => Row::F,
                        e => {
                            return Err(format!(
                                "invalid letter {} at position {}, expected one of A,B,C,D,E or F",
                                e, i
                            ))
                        }
                    },
                    match n {
                        '1' => Column::One,
                        '2' => Column::Two,
                        '3' => Column::Tree,
                        '4' => Column::Four,
                        '5' => Column::Five,
                        '6' => Column::Six,
                        e => {
                            return Err(format!(
                                "invalid number {} at position {}, expected one of 1,2,3,4,5 or 6",
                                e, j
                            ))
                        }
                    },
                ))
            });
        let res = Dices([
            iter.next().ok_or("Missing Dice".to_string())??,
            iter.next().ok_or("Missing Dice".to_string())??,
            iter.next().ok_or("Missing Dice".to_string())??,
            iter.next().ok_or("Missing Dice".to_string())??,
            iter.next().ok_or("Missing Dice".to_string())??,
            iter.next().ok_or("Missing Dice".to_string())??,
            iter.next().ok_or("Missing Dice".to_string())??,
        ]);
        let next = iter.next();
        println!("got dices:{:?}, next: {:?}", res, next);
        match next {
            Some(_) => Err("To many DiceRols".to_string()),
            None => Ok(res),
        }
    }
}
#[derive(Parser, Debug)]
struct PartialArgs {
    dieces: Option<Dices>,
}
struct Args {
    dieces: Dices,
}
impl Args {
    fn parse() -> Self {
        let partial = PartialArgs::parse();
        Self {
            dieces: partial.dieces.unwrap_or_else(|| dices::get_dices()),
        }
    }
}
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
            self.vecs[0][self.indices[0]].clone(),
            self.vecs[1][self.indices[1]].clone(),
            self.vecs[2][self.indices[2]].clone(),
            self.vecs[3][self.indices[3]].clone(),
            self.vecs[4][self.indices[4]].clone(),
            self.vecs[5][self.indices[5]].clone(),
            self.vecs[6][self.indices[6]].clone(),
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

        Some(Dices(result))
    }
}
struct BoardIterator {
    vec: [usize; 7],
}

impl BoardIterator {
    fn new() -> Self {
        BoardIterator {
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
        for (i, revi) in (0..moved).into_iter().rev().enumerate() {
            self.vec[6 - revi] = self.vec[6 - moved] + i + 1;
        }
        Some(Board::from_pos(&self.vec))
    }
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::{Board, Column, Dices, Row};

    use super::dices::*;
    use test::Bencher;

    #[bench]
    fn find_all_solutions_to_all_dice_rolls(b: &mut Bencher) {
        b.iter(|| {
            Board::new(get_dices()).solve();
        })
    }
}
fn main() {
    use indicatif::ProgressIterator;
    let iterator = BoardIterator::new();
    let mut all_solutions: usize = iterator
        .par_bridge()
        .progress_count(8_347_680 / 16)
        .map(|board| board.solve().len())
        .sum(); // Process the board as needed

    println!("all solutions:{}", all_solutions);

    /* let args = Args::parse();

    let mut starter_board = BitMap64::new(0);
    for (r, c) in args.dieces.0 {
        starter_board.set_bit(r as u64 * 8 + c as u64);
    }
    let board = Board::new(args.dieces);
    let solutions = board.solve();

    println!("starterboard:\n{:?}\n{}", starter_board, solutions[0]);
    println!("amount of solutions fund: {}", solutions.len()) */
}
#[derive(Debug, Clone, Copy)]
struct Board {
    starter_board: BitMap64,
}

impl Board {
    fn from_pos(vec: &[usize; 7]) -> Self {
        let mut starter_board = BitMap64::new(0);
        for i in vec {
            starter_board.set_bit((i / 6 * 8 + i % 6) as u64)
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
        let mut possible = pieces::get_possible(self.starter_board.get_copied_inner());
        let pre_candidates = possible
            .pop()
            .expect("it should allways have 9 in the thingymagig");
        //println!("allocating {}u64s", pre_candidates.len() * 10);
        let mut candidates: Vec<_> = pre_candidates
            .into_iter()
            .map(|v| PieceBoard::new(v))
            .collect();

        for (i, piece_positions) in possible.iter().rev().enumerate() {
            //println!("being iteration: {}, candidates:{}", i, candidates.len());
            candidates = piece_positions
                .par_iter()
                .flat_map_iter(|v| candidates.iter().filter_map(|val| val.try_insert(v, i)))
                .collect();
        }

        candidates
    }
}
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use pieces::Pieces::*;
        for (i, piece) in [
            OneByOne, OneByTwo, OneByThree, OneByFour, TwoByTwo, Shape6, Shape7, Shape8, Shape9,
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use pieces::Pieces::*;
        for (i, piece) in [
            OneByOne, OneByTwo, OneByThree, OneByFour, TwoByTwo, Shape6, Shape7, Shape8, Shape9,
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
    #[inline]
    pub fn get_possible(bitmap: u64) -> Vec<Vec<BitMap64>> {
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
        .map(|(index, piece)| {
            let mut arr = piece.get_array();
            arr.retain(|v| (*v & bitmap).get_copied_inner() == 0);
            arr
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
        fn get_array(&self) -> Vec<BitMap64> {
            match self {
                Pieces::OneByOne => OneByOne.into_iter().flatten().collect(),
                Pieces::OneByTwo => OneByTwo.into_iter().flatten().collect(),
                Pieces::OneByThree => OneByThree.into_iter().flatten().collect(),
                Pieces::OneByFour => OneByFour.into_iter().flatten().collect(),
                Pieces::TwoByTwo => TwoByTwo.into_iter().flatten().collect(),
                Pieces::Shape6 => Shape6.into_iter().flatten().collect(),
                Pieces::Shape7 => Shape7.into_iter().flatten().collect(),
                Pieces::Shape8 => Shape8.into_iter().flatten().collect(),
                Pieces::Shape9 => Shape9.into_iter().flatten().collect(),
            }
        }
    }
}
