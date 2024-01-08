use crate::pnum::PNum;

#[repr(u16)]
/// Speedの適用順番や処理は不明なので適当
pub enum SpeedAppliers{
	クオークチャージ速,
	こだわりスカーフ,
}

impl SpeedAppliers{
	pub fn value(&self) -> PNum{
		use SpeedAppliers as S;
		use crate::pnum::PNum as P;
		match self{
			S::クオークチャージ速 => P::V1_5,
			S::こだわりスカーフ => P::V1_5,
		}
	}

	pub fn from_u16(v : u16) -> SpeedAppliers{
		unsafe{ std::mem::transmute(v) }
	}
}