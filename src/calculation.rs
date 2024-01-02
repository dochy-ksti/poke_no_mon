use crate::{atk_appliers::AtkAppliers, power_appliers::PowerAppliers, def_appliers::DefAppliers, damage_appliers::DamageAppliers, types::Types, def_types::DefTypes};



pub fn calculation_main(level : usize, move_power : usize, atk : usize, def : usize, 
	move_type : Types, atk_type_boost : bool, teras_boost : bool, def_types : DefTypes,
	power_appliers: &[PowerAppliers], atk_appliers : &[AtkAppliers], 
	def_appliers : &[DefAppliers], damage_appliers : &[DamageAppliers]){
	let d = (level * 2) / 5 + 2;
	let d = d * move_power * atk / def;
	let d = d / 50 + 2;
	
}