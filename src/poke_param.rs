use crate::{
    abilities::Abilities, def_types::DefTypes, items::Items, poke_move::PokeMove, pokemon::Stats,
};

pub struct PokeParam {
    /// name は immutable (だと思う)
    pub name: String,
	/// level は immutable (だと思う)
	pub level: u32,
    /// stats は immutable (だと思う)
    pub stats: Stats,
	/// weight は immutable (だと思う)
	pub weight : u32,
    /// type は mutable
    pub def_types: DefTypes,

    /// とくせい は mutable
    pub ability: Abilities,

    /// item はもちろん mutable
    pub item: Option<Items>,

    /// move は基本的にはimmutableなはず。わるあがきはあるが・・・
    pub moves: Vec<PokeMove>,

    pub atk_rank: i32,
    pub def_rank: i32,
    pub satk_rank: i32,
    pub sdef_rank: i32,
    pub speed_rank: i32,
	pub accuracy_rank: i32,
	pub evasion_rank:i32,
  
}
