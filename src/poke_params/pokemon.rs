use super::def_types::DefTypes;


pub struct Pokemon{
	pub name : String,
	pub base : BaseStats,
	pub iv : IndivisualValues,
	pub ef : EffortValues,

	pub def_types : DefTypes,
	pub weight : u32,
	
	pub level : u16,
	pub natures : Natures,

	//とくせいは別枠にする
}

impl Pokemon{
	pub fn calc_stats(&self) -> Stats{
		let mut stats : Stats = Default::default();

		let hp = (self.base.hp() * 2 + self.iv.hp() + self.ef.hp()/4) * self.level / 100 + 10 + self.level;
		stats.values[0] = hp;

		for n in 1..6{
			let base = self.base.values[n];
			let iv = self.iv.values[n];
			let ef = self.ef.values[n];
			let nature = self.natures.values[n];
			let v = (((base * 2 + iv + ef/4) * self.level) / 100 + 5) * nature / 10;
			stats.values[n] = v;
		}
		stats
	}
}


#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum StatKind{
	Hp, Atk, Def, SAtk, SDef, Speed
}

/// せいかく
pub struct Natures{
	/// 0 は使わない
	pub values : [u16; 6]
}

impl Default for Natures{
    fn default() -> Self {
        Self { values: [10,10,10,10,10,10] }
    }
}

impl Natures{
	pub fn new(up : StatKind, down : StatKind) -> Natures{
		if up == StatKind::Hp || down == StatKind::Hp{
			panic!("Nature can't be applied to HP")
		}
		let mut n : Natures = Default::default();

		n.values[up as u8 as usize] = 11;
		n.values[down as u8 as usize] = 9;

		n
	}
}

/// ステータス
#[derive(Debug, Default)]
pub struct Stats{
	pub values : [u16; 6]
}

impl Stats{
	pub fn hp(&self) -> u16{ self.values[0] }
	pub fn atk(&self) -> u16{ self.values[1] }
	pub fn def(&self) -> u16{ self.values[2] }
	pub fn satk(&self) -> u16{ self.values[3] }
	pub fn sdef(&self) -> u16{ self.values[4] }
	pub fn speed(&self) -> u32{ self.values[5] as u32 }
}

/// 個体値
pub struct IndivisualValues{
	pub values : [u16; 6]
}

impl IndivisualValues{
	pub fn hp(&self) -> u16{ self.values[0] }
	pub fn atk(&self) -> u16{ self.values[1] }
	pub fn def(&self) -> u16{ self.values[2] }
	pub fn satk(&self) -> u16{ self.values[3] }
	pub fn sdef(&self) -> u16{ self.values[4] }
	pub fn speed(&self) -> u16{ self.values[5] }
}

/// 努力値
pub struct EffortValues{
	pub values : [u16; 6]
}

impl EffortValues{
	pub fn hp(&self) -> u16{ self.values[0] }
	pub fn atk(&self) -> u16{ self.values[1] }
	pub fn def(&self) -> u16{ self.values[2] }
	pub fn satk(&self) -> u16{ self.values[3] }
	pub fn sdef(&self) -> u16{ self.values[4] }
	pub fn speed(&self) -> u16{ self.values[5] }
}

/// 種族値
pub struct BaseStats{
	pub values : [u16; 6]
}

impl BaseStats{
	pub fn hp(&self) -> u16{ self.values[0] }
	pub fn atk(&self) -> u16{ self.values[1] }
	pub fn def(&self) -> u16{ self.values[2] }
	pub fn satk(&self) -> u16{ self.values[3] }
	pub fn sdef(&self) -> u16{ self.values[4] }
	pub fn speed(&self) -> u16{ self.values[5] }
}