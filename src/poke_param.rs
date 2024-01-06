use crate::{
    abilities::Abilities, def_types::DefTypes, items::Items, poke_move::PokeMove, pokemon::Stats,
};

pub struct PokeParam {
    /// name は immutable (だと思う)
    pub name: String,
    /// stats は immutable
    pub stats: Stats,
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
    //命中率や択を扱う方法は分かってない。いやわかってはいるんだが、複雑すぎて実装できる気がしない・・・
}
