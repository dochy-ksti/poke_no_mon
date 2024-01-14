use crate::{
    ability_items::{abilities::Abilities, items::Items},
    moves::unique_move::PokeMove,
    simulations::calc_rank::calc_rank,
};

use super::{def_types::DefTypes, pokemon::Stats, types::Types};

pub struct PokeParam {
    /// name は immutable (だと思う)
    pub name: String,
    /// level は immutable (だと思う)
    pub level: u32,
    /// stats は mutable(メテノとか、なんとかスワップとかで実数値を直接変更することがある)
    pub stats: Stats,
    /// weight は ボディパージで変化するようだ。かるいしをはたき落とされたりもありうるか。
    pub weight: u32,
    /// type は mutable。みずびたしとか
    pub def_types: DefTypes,

    /// とくせい は mutable。スキルスワップとか。
    pub ability: Abilities,

    /// item はもちろん mutable。はたきおとされたり交換されたり。
    pub item: Items,

    /// move は基本的にはimmutableなはず。わるあがきはあるが・・・
    pub moves: Vec<PokeMove>,
	/// 前回使った技
	pub	previously_selected : Option<usize>,
	pub こだわり : bool,

    /// テラスは当然 mutable
    pub teras: Option<Types>,

    pub atk_rank: i32,
    pub def_rank: i32,
    pub satk_rank: i32,
    pub sdef_rank: i32,
    pub speed_rank: i32,
    pub accuracy_rank: i32,
    pub evasion_rank: i32,
    pub critical_rank: i32,

    /// 小さくなっているかどうか
    pub is_small: bool,

    pub boost_energy: ParadoxBoost,
    pub climate_paradox_boost: ParadoxBoost,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ParadoxBoost {
    None,
    Atk,
    Def,
    SAtk,
    SDef,
    Speed,
}

impl PokeParam {
	pub fn atk(&self) -> u32{
		calc_rank(self.stats.atk(), self.atk_rank)
	}
	pub fn def(&self) -> u32{
		calc_rank(self.stats.def(), self.def_rank)
	}
	pub fn satk(&self) -> u32{
		calc_rank(self.stats.satk(), self.satk_rank)
	}
	pub fn sdef(&self) -> u32{
		calc_rank(self.stats.sdef(), self.sdef_rank)
	}

    pub fn speed(&self) -> u32 {
        calc_rank(self.stats.speed(), self.speed_rank)
    }

	pub fn speed_raw(&self) -> u32{
		self.stats.speed()
	}

    fn paradox_boost(&self) -> ParadoxBoost {
        // 両方ともNone以外になるということはない。あってはならない。
        if self.boost_energy == ParadoxBoost::None {
            return self.climate_paradox_boost;
        } else {
            return self.boost_energy;
        }
    }
}
