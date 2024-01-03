use crate::def_types::DefTypes;

pub struct Pokemon{
	pub name : String,
	pub base : BaseStats,
	pub iv : IndivisualValues,
	pub ef : EffortValues,

	pub def_types : DefTypes,
	pub weight : u32,
	
	pub level : u8,
	pub natures : Natures,

	//とくせいは別枠にする
}

impl Pokemon{
	pub fn calc_stats(&self) -> Stats{

	}
}


#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum StatKind{
	Hp, Atk, Def, SAtk, SDef, Speed
}

pub struct Natures{
	/// 0 は使わない
	pub values : [u8; 6]
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

pub struct Stats{
	pub values : [u8; 6]
}

pub struct IndivisualValues{
	pub values : [u8; 6]
}

impl IndivisualValues{
	pub fn hp(&self) -> u8{ self.values[0] }
	pub fn atk(&self) -> u8{ self.values[1] }
	pub fn def(&self) -> u8{ self.values[2] }
	pub fn satk(&self) -> u8{ self.values[3] }
	pub fn sdef(&self) -> u8{ self.values[4] }
	pub fn speed(&self) -> u8{ self.values[5] }
}

pub struct EffortValues{
	pub values : [u8; 6]
}

impl EffortValues{
	pub fn hp(&self) -> u8{ self.values[0] }
	pub fn atk(&self) -> u8{ self.values[1] }
	pub fn def(&self) -> u8{ self.values[2] }
	pub fn satk(&self) -> u8{ self.values[3] }
	pub fn sdef(&self) -> u8{ self.values[4] }
	pub fn speed(&self) -> u8{ self.values[5] }	
}

pub struct BaseStats{
	pub values : [u8; 6]
}

impl BaseStats{
	pub fn hp(&self) -> u8{ self.values[0] }
	pub fn atk(&self) -> u8{ self.values[1] }
	pub fn def(&self) -> u8{ self.values[2] }
	pub fn satk(&self) -> u8{ self.values[3] }
	pub fn sdef(&self) -> u8{ self.values[4] }
	pub fn speed(&self) -> u8{ self.values[5] }	
}