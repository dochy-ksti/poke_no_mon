use crate::{atk_appliers::AtkAppliers, def_appliers::DefAppliers, power_appliers::PowerAppliers, damage_appliers::DamageAppliers};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum ApplierType{
	Power, Atk, Def, Damage
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Applier{
	pub t : ApplierType,
	pub num : u16,
}

impl Applier{
	pub fn power(app : PowerAppliers) -> Applier{
		Applier{ t : ApplierType::Power, num : app as u16 }
	}
	pub fn atk(app : AtkAppliers) -> Applier{
		Applier{ t : ApplierType::Atk, num : app as u16 }
	}
	pub fn def(app : DefAppliers) -> Applier{
		Applier{ t : ApplierType::Def, num : app as u16 }
	}
	pub fn damage(app : DamageAppliers) -> Applier{
		Applier{ t : ApplierType::Damage, num : app as u16 }
	}
}