use crate::{moves::{unique_move::UniqueMove, damage_move::DamageMove}, poke_params::{poke_param::PokeParam, poke_const::PokeConst}};

/// 参考資料
/// https://latest.pokewiki.net/%E3%83%90%E3%83%88%E3%83%AB%E4%B8%AD%E3%81%AE%E5%87%A6%E7%90%86%E3%81%AE%E9%A0%86%E7%95%AA
pub fn calculate_damage(p1: &PokeParam, p1_c : &PokeConst, m : &DamageMove) -> u32 {
    // calculate_damage_main(
    //     level,
    //     move_power,
    //     atk,
    //     def,
    //     atk_rank,
    //     def_rank,
    //     atk_type_boost,
    //     teras_boost,
    //     type_effectiveness,
    //     power_appliers,
    //     atk_appliers,
    //     def_appliers,
    //     damage_appliers,
    // )

    todo!()
}
