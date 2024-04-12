use crate::dices::*;
use clap::Parser;
#[derive(Parser, Debug)]
struct PartialArgs {
    dices: Option<Dices>,
}
pub struct CustomArgs {
    pub dices: Dices,
}
impl CustomArgs {
    pub fn parse() -> Self {
        let partial = PartialArgs::parse();
        Self {
            dices: partial.dices.unwrap_or_else(crate::dices::get_dices),
        }
    }
}
