use crate::pnum::PNum;

#[repr(u16)]
/// この順番にApplyしていくので、順番も重要
pub enum AtkAppliers{
	わざわい器札,
	クオークチャージ攻,
	こだわりハチマキ等,
}

impl AtkAppliers{
	pub fn value(&self) -> PNum{
		use AtkAppliers as A;
		use crate::pnum::PNum as P;
		match self{
			A::わざわい器札 => P::V0_75,
			A::クオークチャージ攻 => P::V1_3,
			A::こだわりハチマキ等 => P::V1_5,
		}
	}

	pub fn from_u16(v : u16) -> AtkAppliers{
		unsafe{ std::mem::transmute(v) }
	}
}