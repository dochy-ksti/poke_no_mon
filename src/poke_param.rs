use crate::{
    abilities::Abilities, def_types::DefTypes, items::Items, unique_move::PokeMove, pokemon::Stats,
    types::Types,
};

pub struct PokeParam {
    /// name は immutable (だと思う)
    pub name: String,
    /// level は immutable (だと思う)
    pub level: u32,
    /// stats は immutable (だと思う)
    pub stats: Stats,
    /// weight は immutable (だと思う)
    pub weight: u32,
    /// type は mutable。みずびたしとか
    pub def_types: DefTypes,

    /// とくせい は mutable。スキルスワップとか。
    pub ability: Abilities,

    /// item はもちろん mutable
    pub item: Option<Items>,

    /// move は基本的にはimmutableなはず。わるあがきはあるが・・・
    pub moves: Vec<PokeMove>,

    /// テラスは当然 mutable
    pub teras: Option<Types>,

    pub atk_rank: i32,
    pub def_rank: i32,
    pub satk_rank: i32,
    pub sdef_rank: i32,
    pub speed_rank: i32,
    pub accuracy_rank: i32,
    pub evasion_rank: i32,
}
