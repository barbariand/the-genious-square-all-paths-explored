use crate::dices::{Column, Dices, Row, DICE1, DICE2, DICE3, DICE4, DICE5, DICE6, DICE7};
pub struct DiceCombinationIterator<'a> {
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
            }
            self.indices[i] = 0;
            if i == 0 {
                self.done = true;
            }
        }

        Some(Dices::new(result))
    }
}
