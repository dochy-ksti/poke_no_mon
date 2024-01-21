use crate::{
    ability_items::{abilities::Abilities, items::Items},
    moves::unique_move::UniqueMove,
    simulations::calc_rank::calc_rank,
};

use super::{def_types::DefTypes, pokemon::Stats, ranks::Ranks, types::Types};

#[derive(Debug, Clone)]
pub struct PokeParam {
    /// name は immutable (だと思う)
    pub name: String,
    /// level は immutable (だと思う)
    pub level: u32,
    /// stats は mutable(メテノとか、なんとかスワップとかで実数値を直接変更することがある)
    pub stats: Stats,
    pub hp: u32,
    /// weight は ボディパージで変化するようだ。かるいしをはたき落とされたりもありうるか。
    pub weight: u32,
    /// type は mutable。みずびたしとか
    pub def_types: DefTypes,

    /// とくせい は mutable。スキルスワップとか。
    pub ability: Abilities,

    /// item はもちろん mutable。はたきおとされたり交換されたり。
    pub item: Items,

    /// 前回使った技
    pub previously_selected: Option<usize>,
    pub こだわり: bool,

    /// テラスは当然 mutable
    pub teras: Option<Types>,

    pub ranks: Ranks,

    /// 小さくなっているかどうか
    pub is_small: bool,

    pub boost_energy: ParadoxBoost,
    pub climate_paradox_boost: ParadoxBoost,
    //状態異常

    //アンコールとか、もろもろの変化。じゅうでんとか、いろいろ必要
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
    pub fn ranked_atk(&self) -> u32 {
        calc_rank(self.stats.atk(), self.ranks.atk())
    }
    pub fn ranked_def(&self) -> u32 {
        calc_rank(self.stats.def(), self.ranks.def())
    }
    pub fn ranked_satk(&self) -> u32 {
        calc_rank(self.stats.satk(), self.ranks.satk())
    }
    pub fn ranked_sdef(&self) -> u32 {
        calc_rank(self.stats.sdef(), self.ranks.sdef())
    }

    pub fn ranked_speed(&self) -> u32 {
        calc_rank(self.stats.speed(), self.ranks.speed())
    }

    pub fn atk(&self) -> u32 {
        self.stats.atk()
    }

    pub fn def(&self) -> u32 {
        self.stats.def()
    }
    pub fn satk(&self) -> u32 {
        self.stats.satk()
    }
    pub fn sdef(&self) -> u32 {
        self.stats.sdef()
    }

    pub fn speed(&self) -> u32 {
        self.stats.speed()
    }

	pub fn atk_rank(&self) -> i32{
		self.ranks.atk()
	}

	pub fn def_rank(&self) -> i32{
		self.ranks.def()
	}

	pub fn satk_rank(&self) -> i32{
		self.ranks.satk()
	}

	pub fn sdef_rank(&self) -> i32{
		self.ranks.sdef()
	}

	pub fn speed_rank(&self) -> i32{
		self.ranks.speed()
	}

	pub fn critical_rank(&self) -> i32{
		self.ranks.critical()
	}
	

    pub fn paradox_boost(&self) -> ParadoxBoost {
        // 両方ともNone以外になるということはない。あってはならない。
        if self.boost_energy == ParadoxBoost::None {
            return self.climate_paradox_boost;
        } else {
            return self.boost_energy;
        }
    }

	pub fn max_hp(&self) -> u32{
		self.stats.hp()
	}

	pub fn restore_hp(&mut self, val : u32){
		self.hp += val;
		if self.max_hp() <  self.hp{
			self.hp = self.max_hp()
		}
	}

	pub fn inflict_damage(&mut self, val : u32){
		if self.hp < val{
			self.hp = 0;
		} else{
			self.hp -= val;
		}
	}

	pub fn apply_rank_delta(&mut self, rank_delta : &Ranks){
		self.ranks.apply_rank_delta(rank_delta);
	}
}
