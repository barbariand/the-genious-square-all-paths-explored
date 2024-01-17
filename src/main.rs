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
mod board;
mod dices;
mod generated_bitboards;
mod iter;
mod pieceboard;
mod pieces;
use bitmap::BitMap64;
use board::Board;

// removes the ones where the start is on any even square visulised bellow with plusses and minus
/*
+-+-+-+-
-+-+-+-+
+-+-+-+-
-+-+-+-+
+-+-+-+-
-+-+-+-+
where it can only be on plus
 */
const INVALID: BitMap64 = BitMap64::new(18_446_720_803_221_662_421);
fn main() {
    // finding all solutions
    // neeeds to be commented out and prob passed in some args or smth
    /*use indicatif::ProgressIterator;
    println!(
        "{}",
        BoardIterator::new()
            .filter(|v| v.starter_board & INVALID == BitMap64::new(0))
            .count()
    );
    todo!();
    let iterator = BoardIterator::new();

    let mut all_solutions: usize = iterator
        .par_bridge()
        .filter(|v| v.starter_board & INVALID == BitMap64::new(0))
        .map(|board| {
            let dieces = board.starter_board.clone();
            let len = board.solve().len();
            println!("dieces:{},solutions{}", dieces.get_copied_inner(), len);
            len
        })
        .sum();

    // Process the board as needed

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
