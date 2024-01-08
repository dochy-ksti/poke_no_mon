use crate::pnum::PNum;

#[repr(u16)]
pub enum DamageAppliers{
	/// 0.5 2048
	マルチスケイル,
	/// 1.3 5324
	いのちのたま,
}

impl DamageAppliers{
	pub fn value(&self) -> PNum{
		use DamageAppliers as D;
		use crate::pnum::PNum as P;
		match self{
			D::マルチスケイル => P::V0_5,
			D::いのちのたま => P::V1_3,
		}
	}

	pub fn from_u16(v : u16) -> DamageAppliers{
		unsafe{ std::mem::transmute(v) }
	}
}