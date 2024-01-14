use crate::pnum::PNum;

//https://wiki.xn--rckteqa2e.com/wiki/%E3%81%99%E3%81%B0%E3%82%84%E3%81%95#%E3%81%99%E3%81%B0%E3%82%84%E3%81%95%E8%A3%9C%E6%AD%A3


#[repr(u16)]
pub enum SpeedAppliers{
	パラドックス,
	こだわりスカーフ,
}

impl SpeedAppliers{
	pub fn value(&self) -> PNum{
		use SpeedAppliers as S;
		use crate::pnum::PNum as P;
		match self{
			S::パラドックス => P::V1_5,
			S::こだわりスカーフ => P::V1_5,
		}
	}

	pub fn from_u16(v : u16) -> SpeedAppliers{
		unsafe{ std::mem::transmute(v) }
	}
}