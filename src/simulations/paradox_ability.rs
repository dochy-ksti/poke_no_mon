use crate::{poke_params::poke_param::{PokeParam, ParadoxBoost}, environment::environment::Environment, ability_items::{abilities::Abilities, items::Items}};

// 参考資料
// https://wiki.xn--rckteqa2e.com/wiki/%E3%83%96%E3%83%BC%E3%82%B9%E3%83%88%E3%82%A8%E3%83%8A%E3%82%B8%E3%83%BC#%E8%A9%B3%E7%B4%B0%E3%81%AA%E4%BB%95%E6%A7%98
pub fn paradox_ability(p : &mut PokeParam, env: &mut Environment){
	if p.ability == Abilities::こだいかっせい{
		//天気が晴れなら活性化
	} else{
		if p.item == Items::ブーストエナジー{
			p.item = Items::なし;
			p.boost_energy = activate(p);
		}
	}
	todo!()
}

fn activate(p : &mut PokeParam) -> ParadoxBoost{

}