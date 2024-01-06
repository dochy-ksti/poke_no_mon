use crate::{
    abilities::Abilities,
    calculate_damage::calculate_damage_main,
    def_types::DefTypes,
    pnum::PNum,
    pokemon::{Pokemon, Stats},
    types::Types, items::Items,
};

pub fn playout(
    attacker: Pokemon,
    attacker_stats: Stats,
    attacker_ability: Abilities,
	attacker_item : Items,
    defender: Pokemon,
    defender_stats: Stats,
	defender_ability:Abilities,
	defender_item : Items,
) {
}
