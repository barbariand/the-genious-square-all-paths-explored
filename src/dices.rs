#[derive(Clone, Copy, Debug)]
pub enum Row {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
}
#[derive(Clone, Copy, Debug)]
pub enum Column {
    One = 0,
    Two = 1,
    Tree = 2,
    Four = 3,
    Five = 4,
    Six = 5,
}

use std::str::FromStr;

use rand::{self, Rng};

use Column::{Five, Four, One, Six, Tree, Two};
use Row::{A, B, C, D, E, F};

/// possible values for dice 1
pub const DICE1: [(Row, Column); 2] = [(F, One), (A, Six)];
/// possible values for dice 2
pub const DICE2: [(Row, Column); 6] = [
    (E, Four),
    (E, Five),
    (E, Six),
    (D, Five),
    (F, Four),
    (F, Five),
];
/// possible values for dice 3
pub const DICE3: [(Row, Column); 6] = [(A, One), (C, One), (D, One), (D, Two), (E, Two), (F, Tree)];
/// possible values for dice 4
pub const DICE4: [(Row, Column); 6] = [
    (A, Four),
    (B, Five),
    (C, Five),
    (C, Six),
    (F, Six),
    (D, Six),
];
/// possible values for dice 5
pub const DICE5: [(Row, Column); 6] =
    [(A, Tree), (B, One), (B, Two), (A, Two), (B, Tree), (C, Two)];
/// possible values for dice 6
pub const DICE6: [(Row, Column); 6] = [
    (B, Four),
    (C, Tree),
    (C, Four),
    (D, Tree),
    (D, Four),
    (E, Tree),
];
/// possible values for dice 7
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

#[derive(Clone, Copy, Debug)]
pub struct Dices(pub [(Row, Column); 7]);
impl Dices {
    pub const fn new(new: [(Row, Column); 7]) -> Dices {
        Dices(new)
    }
}
impl FromStr for Dices {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s
            .chars()
            .enumerate()
            .array_chunks()
            .map(|[(i, a), (j, n), _]| {
                Ok((
                    match a {
                        'A' => Row::A,
                        'B' => Row::B,
                        'C' => Row::C,
                        'D' => Row::D,
                        'E' => Row::E,
                        'F' => Row::F,
                        'a' => Row::A,
                        'b' => Row::B,
                        'c' => Row::C,
                        'd' => Row::D,
                        'e' => Row::E,
                        'f' => Row::F,

                        e => {
                            return Err(format!(
                                "invalid letter {e} at position {i}, expected one of A,B,C,D,E or F"
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
                                "invalid number {e} at position {j}, expected one of 1,2,3,4,5 or 6"
                            ))
                        }
                    },
                ))
            });
        let res = Self([
            iter.next().ok_or("Missing Dice".to_string())??,
            iter.next().ok_or("Missing Dice".to_string())??,
            iter.next().ok_or("Missing Dice".to_string())??,
            iter.next().ok_or("Missing Dice".to_string())??,
            iter.next().ok_or("Missing Dice".to_string())??,
            iter.next().ok_or("Missing Dice".to_string())??,
            iter.next().ok_or("Missing Dice".to_string())??,
        ]);
        let next = iter.next();
        println!("got dices:{res:?}, next: {next:?}");
        match next {
            Some(_) => Err("To many DiceRols".to_string()),
            None => Ok(res),
        }
    }
}
