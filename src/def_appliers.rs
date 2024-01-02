use crate::pnum::PNum;


#[repr(u16)]
pub enum DefAppliers{
	/// 0.75
	わざわい玉剣,

	/// 1.3 5325
	クオークチャージ等,

	/// 1.5 6144
	とつげきチョッキ等,
}

impl DefAppliers{
	pub fn value(&self) -> PNum{
		use DefAppliers as D;
		use crate::pnum::PNum as P;
		match self{
			D::わざわい玉剣 => P::V0_75,
			D::クオークチャージ等 => P::V1_3,
			D::とつげきチョッキ等 => P::V1_5,
		}
	}

	pub fn from_u16(v : u16) -> DefAppliers{
		unsafe{ std::mem::transmute(v) }
	}
}