use crate::{pnum::PNum, poke_params::poke_param::{PokeParam, ParadoxBoost}, environment::environment::Environment, ability_items::items::Items};

//https://wiki.xn--rckteqa2e.com/wiki/%E3%81%99%E3%81%B0%E3%82%84%E3%81%95#%E3%81%99%E3%81%B0%E3%82%84%E3%81%95%E8%A3%9C%E6%AD%A3


///すばやさ補正は特性、アイテム、おいかぜ、しつげんの４つだけ。
/// あとはトリックルームやspeed_rank、まひ補正が影響
#[repr(u16)]
pub enum SpeedAppliers{
	パラドックス,
	こだわりスカーフ,
	おいかぜ,
	しつげん,
}

impl SpeedAppliers{
	pub fn value(&self) -> PNum{
		use SpeedAppliers as S;
		use crate::pnum::PNum as P;
		match self{
			S::パラドックス => P::V1_5,
			S::こだわりスカーフ => P::V1_5,
			S::おいかぜ => P::V2,
			S::しつげん => P::V0_25,
		}
	}

	pub fn from_u16(v : u16) -> SpeedAppliers{
		unsafe{ std::mem::transmute(v) }
	}
}

/// speedに関しては、必要になるタイミングが違うのでapplierで処理しない。
pub fn calc_speed(p : &PokeParam, env : &Environment) -> u32{
	let mut speed = p.speed();
	if p.paradox_boost() == ParadoxBoost::Speed{
		//本当は6144/4092だが、変わらないだろう
		speed = speed * 3 / 2;
	}
	if p.item == Items::こだわりスカーフ{
		speed = speed * 3 / 2;
	}
	//トリックルーム処理やおいかぜ処理、まひ処理など
	speed
}