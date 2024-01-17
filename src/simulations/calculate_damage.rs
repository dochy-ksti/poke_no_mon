use crate::{
    moves::{
        damage_move::{DamageMove, MoveKind},
        unique_move::UniqueMove,
    },
    poke_params::{poke_const::PokeConst, poke_param::PokeParam}, appliers::{applier::Applier, calc_appliers::calc_appliers},
};

use super::calculate_damage_main::{calculate_damage_main, CalcMainResult};

/// 参考資料
/// https://latest.pokewiki.net/%E3%83%90%E3%83%88%E3%83%AB%E4%B8%AD%E3%81%AE%E5%87%A6%E7%90%86%E3%81%AE%E9%A0%86%E7%95%AA
pub fn calculate_damage(
    p1: &PokeParam,
    p1_c: &PokeConst,
    move_power: u32,
    m: &DamageMove,
    p2: &PokeParam,
) -> CalcMainResult {
    let level = p1_c.level;
    let (atk, def) = match m.kind {
        MoveKind::Physical => (p1.atk(), p2.def()),
        MoveKind::Special => (p1.satk(), p2.sdef()),
        MoveKind::Status => panic!("Damages can't be calculated for status moves"),
    };
    let atk_type_boost = p1.def_types.contains(m.move_type);
    let teras_boost = if let Some(t) = p1.teras {
        t == m.move_type
    } else {
        false
    };
	let type_effectiveness = p2.def_types.effectiveness(m.move_type);

	let mut appliers : Vec<Applier> = vec![];

	add(&mut appliers, p1.ability.attacker(p1, m.kind, p2));
	add(&mut appliers, p2.ability.defender(p2, m.kind, p1));
	add(&mut appliers, p1.item.attacker(p1, m.kind, p2));
	add(&mut appliers, p2.item.defender(p2, m.kind, p1));

	let app = calc_appliers(&appliers);

    calculate_damage_main(
        level,
        move_power,
        atk,
        def,
        atk_type_boost,
        teras_boost,
        type_effectiveness,
		&app.power,
        &app.atk,
        &app.def,
        &app.damage,
    )
}

fn add(appliers : &mut Vec<Applier>, app : Option<Applier>){
	if let Some(app) = app{
		appliers.push(app)
	}
}