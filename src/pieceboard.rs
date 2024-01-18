use crate::bitmap;
use crate::bitmap::BitMap64;
use std::fmt::format;
use std::fmt::Debug;
use std::fmt::Display;
use std::sync::Arc;

/* pub struct PieceBoard {
    pub total: BitMap64,
    pieces: [BitMap64; 9],
}
impl PieceBoard {
    pub fn try_insert(&self, new: &BitMap64, i: usize) -> Option<Self> {
        (&self.total & new == BitMap64::new(0)).then(|| {
            let mut pieces = self.pieces.clone();
            pieces[i + 1] = new.clone();
            Self {
                total: &self.total | new,
                pieces,
            }
        })
    }
    #[inline(always)]
    pub fn insert(&self, new: BitMap64, total: BitMap64, i: usize) -> Self {
        let mut pieces = self.pieces.clone();
        pieces[i + 1] = new;
        Self { total, pieces }
    }
    pub fn new(first: BitMap64) -> Self {
        Self {
            total: first.clone(),
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
        use crate::pieces::Pieces::{
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
        use crate::pieces::Pieces::{
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
} */
#[derive(Clone)]
pub struct StartNode(pub BitMap64);

// really bad name but good for testing rn
pub trait TryInsertNode {
    fn try_insert(&self, new: BitMap64) -> Option<Arc<Nodes>>;
    fn insert(&self, new: BitMap64, total: BitMap64) -> Arc<Nodes>;
    fn insert_as_total(&self, new: BitMap64) -> Arc<Nodes>;
}
#[derive(Clone)]
pub enum Nodes {
    PieceBoardNode(PieceBoardNode),
    StartNode(StartNode),
}

impl Nodes {
    pub fn total<'a>(&'a self) -> &'a BitMap64 {
        match self {
            Nodes::PieceBoardNode(v) => &v.total,
            Nodes::StartNode(v) => &v.0,
        }
    }
}
#[derive(Clone)]
pub struct PieceBoardNode {
    pub total: BitMap64,
    current: BitMap64,
    previous: Arc<Nodes>,
}

impl TryInsertNode for Arc<Nodes> {
    #[inline(always)]
    fn try_insert(&self, bitmap: BitMap64) -> Option<Arc<Nodes>> {
        let total = self.total();
        (total & &bitmap == BitMap64::new(0)).then(|| {
            Arc::new(Nodes::PieceBoardNode(PieceBoardNode {
                total: total | &bitmap,
                previous: self.clone(),
                current: bitmap,
            }))
        })
    }
    #[inline(always)]
    fn insert(&self, new: BitMap64, total: BitMap64) -> Arc<Nodes> {
        Arc::new(Nodes::PieceBoardNode(PieceBoardNode {
            total: total,
            current: new,
            previous: self.clone(),
        }))
    }
    #[inline(always)]
    fn insert_as_total(&self, new: BitMap64) -> Arc<Nodes> {
        Arc::new(Nodes::PieceBoardNode(PieceBoardNode {
            total: new.clone(),
            current: new,
            previous: self.clone(),
        }))
    }
}
impl Display for Nodes {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use crate::pieces::Pieces::{
            OneByFour, OneByOne, OneByThree, OneByTwo, Shape6, Shape7, Shape8, Shape9, TwoByTwo,
        };
        let a = self.into_iter();

        for (piece, board) in [
            OneByOne, OneByTwo, OneByThree, TwoByTwo, Shape6, Shape7, Shape8, Shape9, OneByFour,
        ]
        .iter()
        .map(|v| format!("{:?}", v))
        .rev()
        .chain(["StarterBoard".to_owned()])
        .zip(a)
        {
            writeln!(
                f,
                "{}:\n{:?}",
                piece,
                match board.as_ref() {
                    Nodes::PieceBoardNode(v) => v.current.clone(),
                    Nodes::StartNode(v) => v.0.clone(),
                }
            )?;
        }
        Ok(())
    }
}
impl Debug for Nodes {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use crate::pieces::Pieces::{
            OneByFour, OneByOne, OneByThree, OneByTwo, Shape6, Shape7, Shape8, Shape9, TwoByTwo,
        };
        let a = self.into_iter();

        for (piece, board) in [
            OneByOne, OneByTwo, OneByThree, TwoByTwo, Shape6, Shape7, Shape8, Shape9, OneByFour,
        ]
        .iter()
        .map(|v| format!("{:?}", v))
        .rev()
        .chain(["StarterBoard".to_owned()])
        .zip(a)
        {
            writeln!(
                f,
                "{}:\n{:?}",
                piece,
                match board.as_ref() {
                    Nodes::PieceBoardNode(v) => v.current.clone(),
                    Nodes::StartNode(v) => v.0.clone(),
                }
            )?;
        }
        Ok(())
    }
}
impl<'a> IntoIterator for &'a Nodes {
    type Item = Arc<Nodes>;

    type IntoIter = NodesIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        NodesIterator {
            depth: 0,
            start: self,
        }
    }
}
pub struct NodesIterator<'a> {
    depth: usize,
    start: &'a Nodes,
}
impl<'a> Iterator for NodesIterator<'a> {
    type Item = Arc<Nodes>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.depth == 0 {
            self.depth += 1;
            return Some(Arc::new(self.start.clone()));
        }
        let res = match self.start {
            Nodes::PieceBoardNode(v) => recursive(&v.previous, self.depth - 1),
            Nodes::StartNode(v) => None,
        };
        self.depth += 1;
        res
    }
}
fn recursive(nodes: &Arc<Nodes>, i: usize) -> Option<Arc<Nodes>> {
    if i == 0 {
        return Some(nodes.clone());
    }
    match nodes.as_ref() {
        Nodes::PieceBoardNode(v) => recursive(&v.previous, i - 1),
        Nodes::StartNode(_) => None,
    }
}
