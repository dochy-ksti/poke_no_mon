use crate::{atk_appliers::AtkAppliers, power_appliers::PowerAppliers, def_appliers::DefAppliers, damage_appliers::DamageAppliers, types::Types, def_types::DefTypes, pnum::PNum};

pub fn calc(){}

pub fn calculation_main(level : u32, move_power : u32, atk : u32, def : u32, 
	atk_rank : i32, def_rank : i32,
	move_type : Types, atk_type_boost : bool, teras_boost : bool, def_types : DefTypes,
	power_appliers: &[PNum], atk_appliers : &[PNum], 
	def_appliers : &[PNum], damage_appliers : &[PNum]){
	let d = (level * 2) / 5 + 2;
	let power = calc_power(move_power, power_appliers, teras_boost);
	let d = d * move_power * atk / def;
	let d = d / 50 + 2;
	
}

fn calc_power(move_power : u32, power_appliers: &[PNum], teras_boost : bool) -> u32{
	let mut power = calc_applier(power_appliers).apply5(move_power);
	if power == 0{ power = 1; }
	if teras_boost && power < 60{
		power = 60;
	}
	power
}

fn calc_applier(appliers : &[PNum]) -> PNum{
	let mut v = PNum::V1;
	for applier in appliers{
		v = v.apply4(*applier);
	}
	v
}