use crate::dices::*;
use clap::Parser;
#[derive(Parser, Debug)]
struct PartialArgs {
    dieces: Option<Dices>,
}
pub struct CustomArgs {
    pub dieces: Dices,
}
impl CustomArgs {
    pub fn parse() -> Self {
        let partial = PartialArgs::parse();
        Self {
            dieces: partial.dieces.unwrap_or_else(crate::dices::get_dices),
        }
    }
}
