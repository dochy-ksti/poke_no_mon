use crate::{
    abilities::Abilities,
    calculate_damage_main::calculate_damage_main,
    def_types::DefTypes,
    items::Items,
    pnum::PNum,
    poke_param::PokeParam,
    pokemon::{Pokemon, Stats},
    types::Types,
};

pub fn playout(p1: PokeParam, p2: PokeParam) {}

pub fn turn(p1: &mut PokeParam, p1_move_index: usize, p2: &mut PokeParam, p2_move_index: usize) {}
