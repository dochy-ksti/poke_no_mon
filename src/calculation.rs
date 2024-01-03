use crate::{calculation_main::calculation_main, def_types::DefTypes, pnum::PNum, types::Types};

pub fn calculate(
    level: u32,
    move_power: u32,
    atk: u32,
    def: u32,
    atk_rank: i32,
    def_rank: i32,
    move_type: Types,
    atk_type_boost: bool,
    teras_boost: bool,
    def_types: DefTypes,
    power_appliers: &[PNum],
    atk_appliers: &[PNum],
    def_appliers: &[PNum],
    damage_appliers: &[PNum],
) {
    calculation_main(
        level,
        move_power,
        atk,
        def,
        atk_rank,
        def_rank,
        move_type,
        atk_type_boost,
        teras_boost,
        def_types,
        power_appliers,
        atk_appliers,
        def_appliers,
        damage_appliers,
    );
}
