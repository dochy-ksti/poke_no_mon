use crate::moves::{unique_move::UniqueMove, poke_move::PokeMove};

#[derive(Debug)]
pub struct PokeConst {
    pub name: String,
    pub level: u32,
    pub moves: Vec<PokeMove>,
}
