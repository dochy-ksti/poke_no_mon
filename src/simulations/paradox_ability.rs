use crate::{
    ability_items::{abilities::Abilities, items::Items},
    environment::environment::Environment,
    poke_params::poke_param::{ParadoxBoost, PokeParam},
};

// 参考資料
// https://wiki.xn--rckteqa2e.com/wiki/%E3%83%96%E3%83%BC%E3%82%B9%E3%83%88%E3%82%A8%E3%83%8A%E3%82%B8%E3%83%BC#%E8%A9%B3%E7%B4%B0%E3%81%AA%E4%BB%95%E6%A7%98
pub fn paradox_ability(p: &mut PokeParam, env: &mut Environment) {
    if p.ability == Abilities::こだいかっせい {
        //天気が晴れなら活性化
    } else {
        if p.item == Items::ブーストエナジー {
            p.item = Items::なし;
            p.boost_energy = activate(p);
        }
    }
	if p.ability == Abilities::クオークチャージ{
		//エレキフィールドなら活性化
	} else{
		if p.item == Items::ブーストエナジー {
            p.item = Items::なし;
            p.boost_energy = activate(p);
        }
	}
}

fn activate(p: &mut PokeParam) -> ParadoxBoost {
    let c: [(ParadoxBoost, u32); 5] = [
        (ParadoxBoost::Speed, p.speed()),
        (ParadoxBoost::SDef, p.sdef()),
        (ParadoxBoost::SAtk, p.satk()),
        (ParadoxBoost::Def, p.def()),
        (ParadoxBoost::Atk, p.atk()),
    ];
	return c.into_iter().max_by_key(|a| a.1).unwrap().0;
	
}
