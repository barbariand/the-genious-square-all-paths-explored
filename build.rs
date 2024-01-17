use proc_macro2::*;
use quote::{quote, ToTokens};

use std::fs::File;
use std::io::Write;
use std::path::Path;

// Assuming BitMap64 and Piece are defined elsewhere
// Define functions to calculate bitboards...
fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generated_bitboards.rs");
    let mut file = File::create(dest_path).unwrap();

    let mut tokens = TokenStream::new();
    tokens.extend(quote!(
        use crate::bitmap::BitMap64;
    ));

    for piece in &[
        Piece::OneByOne,
        Piece::OneByTwo,
        Piece::OneByThree,
        Piece::OneByFour,
        Piece::TwoByTwo,
        Piece::Shape6,
        Piece::Shape7,
        Piece::Shape8,
        Piece::Shape9,
    ] {
        let piece_name = Ident::new(&(format!("{:?}", piece).to_uppercase()), Span::call_site());

        let rotations = piece.rotations();
        let mut all_rotations_tokens = Vec::new();

        for rotation in rotations {
            for x in 0..6 {
                for y in 0..6 {
                    if let Some(bitboard) = generate_bitboard_for_piece(&rotation, (x, y)) {
                        let num = bitboard.0;
                        all_rotations_tokens.push(quote! { BitMap64::new(#num)});
                    }
                }
            }
        }

        println!("\n\n shape:{:?}", piece);
        let len = all_rotations_tokens.len();

        tokens.extend(quote! {
            pub const #piece_name: [BitMap64; #len] = [#(#all_rotations_tokens),*];
        });
    }

    println!("cargo:rerun-if-changed=build.rs");
    writeln!(file, "{}", tokens).unwrap();
}
fn generate_bitboard_for_piece(
    rotation: &[(isize, isize)],
    position: (usize, usize),
) -> Option<BitMap64> {
    let mut bitboard = BitMap64::new(0); // Assuming a constructor for BitMap64

    for &(dx, dy) in rotation {
        let (new_x, new_y) = (position.0 as isize + dx, position.1 as isize + dy);

        // Check if the new position is outside the 6x6 area
        if !(0..6).contains(&new_x) || !(0..6).contains(&new_y) {
            return None;
        }

        // Calculate index for the 8x8 board and set the bit
        let index = (new_x + new_y * 8) as usize;
        bitboard.set_bit(index as u64);
    }

    Some(bitboard)
}
#[derive(Debug)]
enum Piece {
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

impl Piece {
    fn rotations(&self) -> Vec<Vec<(isize, isize)>> {
        match *self {
            Piece::OneByOne => vec![vec![(0, 0)]],
            Piece::OneByTwo => vec![vec![(0, 0), (1, 0)], vec![(0, 0), (0, 1)]],
            Piece::OneByThree => vec![vec![(0, 0), (1, 0), (2, 0)], vec![(0, 0), (0, 1), (0, 2)]],
            Piece::OneByFour => vec![
                vec![(0, 0), (1, 0), (2, 0), (3, 0)],
                vec![(0, 0), (0, 1), (0, 2), (0, 3)],
            ],
            Piece::TwoByTwo => vec![vec![(0, 0), (1, 0), (0, 1), (1, 1)]],
            Piece::Shape6 => vec![
                vec![(0, 0), (1, 0), (1, 1), (1, 2)],
                vec![(0, 1), (1, 1), (2, 1), (2, 0)],
                vec![(0, 0), (0, 1), (0, 2), (1, 2)],
                vec![(0, 0), (0, 1), (1, 1), (2, 1)],
                vec![(0, 0), (1, 0), (0, 1), (0, 2)],
                vec![(0, 0), (1, 0), (2, 1), (2, 0)],
                vec![(1, 0), (1, 1), (0, 2), (1, 2)],
                vec![(0, 0), (0, 1), (1, 0), (2, 0)],
            ],
            Piece::Shape7 => vec![
                vec![(0, 0), (1, 0), (1, 1)],
                vec![(1, 0), (0, 1), (1, 1)],
                vec![(0, 0), (1, 1), (0, 1)],
                vec![(0, 0), (1, 0), (0, 1)],
            ],
            Piece::Shape8 => vec![
                vec![(1, 0), (0, 1), (1, 1), (0, 2)],
                vec![(0, 0), (1, 0), (1, 1), (2, 1)],
                vec![(0, 0), (0, 1), (1, 1), (1, 2)],
                vec![(0, 1), (1, 0), (1, 1), (2, 0)],
            ],
            Piece::Shape9 => vec![
                vec![(1, 0), (0, 1), (1, 1), (1, 2)],
                vec![(0, 0), (0, 1), (1, 1), (0, 2)],
                vec![(0, 0), (1, 0), (1, 1), (2, 0)],
                vec![(0, 1), (1, 0), (1, 1), (2, 1)],
            ],
        }
    }
}
use std::ops::*;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BitMap64(u64);

impl BitMap64 {
    pub const fn new(num: u64) -> BitMap64 {
        BitMap64(num)
    }
    pub fn set_bit(&mut self, bit: u64) {
        self.0 |= 1 << bit;
    }

    pub fn clear_bit(&mut self, bit: u64) {
        self.0 &= !(1 << bit);
    }

    pub fn get_bit(&self, bit: u64) -> bool {
        (self.0 & (1 << bit)) != 0
    }
    pub fn get_bit_value(&self, bit: u64) -> u64 {
        (self.0 & (1 << bit)) >> bit
    }
    pub fn contains(&self, bit: u64) -> bool {
        self.get_bit(bit)
    }
    pub fn count_ones(&self) -> u8 {
        self.0.count_ones() as u8
    }
    pub fn get_copied_inner(&self) -> u64 {
        self.0
    }
}
impl ToTokens for BitMap64 {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let formatted_string = format!("BitMap64::new({})", self.0);
        tokens.extend(
            [syn::parse_str::<TokenStream>(&formatted_string).unwrap()]
                .iter()
                .cloned(),
        );
    }
}
impl Shr<u64> for BitMap64 {
    type Output = Self;
    fn shr(self, rhs: u64) -> Self::Output {
        BitMap64(self.0 >> rhs)
    }
}
impl ShrAssign<u64> for BitMap64 {
    fn shr_assign(&mut self, rhs: u64) {
        self.0 = self.0 >> rhs
    }
}
impl ShlAssign<u64> for BitMap64 {
    fn shl_assign(&mut self, rhs: u64) {
        self.0 = self.0 << rhs
    }
}
impl Shl<u64> for BitMap64 {
    type Output = Self;
    fn shl(self, rhs: u64) -> Self::Output {
        BitMap64(self.0 << rhs)
    }
}
impl BitAnd for BitMap64 {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        BitMap64(self.0 & rhs.0)
    }
}
impl BitAndAssign for BitMap64 {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 = self.0 & rhs.0
    }
}
impl BitOr for BitMap64 {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        BitMap64(self.0 | rhs.0)
    }
}
impl BitOrAssign for BitMap64 {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 = self.0 | rhs.0
    }
}
impl BitXor for BitMap64 {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        BitMap64(self.0 ^ rhs.0)
    }
}
impl BitXorAssign for BitMap64 {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 = self.0 ^ rhs.0
    }
}

impl BitAnd<u64> for BitMap64 {
    type Output = Self;
    fn bitand(self, rhs: u64) -> Self::Output {
        BitMap64(self.0 & rhs)
    }
}
impl BitAndAssign<u64> for BitMap64 {
    fn bitand_assign(&mut self, rhs: u64) {
        self.0 = self.0 & rhs
    }
}
impl BitOr<u64> for BitMap64 {
    type Output = Self;
    fn bitor(self, rhs: u64) -> Self::Output {
        BitMap64(self.0 | rhs)
    }
}
impl BitOrAssign<u64> for BitMap64 {
    fn bitor_assign(&mut self, rhs: u64) {
        self.0 = self.0 | rhs
    }
}
impl BitXor<u64> for BitMap64 {
    type Output = Self;
    fn bitxor(self, rhs: u64) -> Self::Output {
        BitMap64(self.0 ^ rhs)
    }
}
impl BitXorAssign<u64> for BitMap64 {
    fn bitxor_assign(&mut self, rhs: u64) {
        self.0 = self.0 ^ rhs
    }
}

impl IntoIterator for &BitMap64 {
    type Item = bool;

    type IntoIter = BitMap64Iterator;

    fn into_iter(self) -> Self::IntoIter {
        BitMap64Iterator {
            bitmap: *self, //copy semantics, it is safe to just move it here
            current: 0,
        }
    }
}
pub struct BitMap64Iterator {
    bitmap: BitMap64,
    current: u64,
}
impl Iterator for BitMap64Iterator {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current > 64 {
            return None;
        }
        let res = self.bitmap.get_bit(self.current);
        self.current += 1;
        Some(res)
    }
}
