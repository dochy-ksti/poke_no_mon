use crate::{poke_params::{poke_const::PokeConst, poke_param::PokeParam}, ability_items::items::Items};

use super::{damage_move::DamageMove, unique_move::UniqueMove};

pub fn calc_varying_power(
    p1: &PokeParam,
    p1_c: &PokeConst,
    m: &DamageMove,
    p2: &PokeParam,
    p2_c: &PokeConst,
) -> u32{
	match m.unique_move{
		UniqueMove::アクロバット =>{
			if p1.item == Items::なし{
				m.power * 2
			} else{
				m.power
			}
		}
		_ => m.power,
	}
}
