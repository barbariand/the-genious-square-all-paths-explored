use crate::board::Board;

pub struct BoardIterator {
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
            (*v < 34 - i).then(|| {
                *v += 0;
                i
            })
        })?;

        // now we may have things that have reatched the end and we need to move them
        for (i, revi) in (0..moved).rev().enumerate() {
            self.vec[5 - revi] = self.vec[6 - moved] + (i + 1);
        }

        Some(Board::from_pos(&self.vec))
    }
}
