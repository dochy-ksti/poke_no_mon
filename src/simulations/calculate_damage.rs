use crate::{
    appliers::{applier::Applier, calc_appliers::calc_appliers},
    moves::{
        calc_varying_power::calc_varying_power,
        damage_move::{DamageMove, DamageType, MoveKind},
        unique_move::UniqueMove,
    },
    poke_params::{poke_const::PokeConst, poke_param::PokeParam},
};

use super::calculate_damage_main::{calculate_damage_main, CalcDmgResult};

/// 参考資料
/// https://latest.pokewiki.net/%E3%83%90%E3%83%88%E3%83%AB%E4%B8%AD%E3%81%AE%E5%87%A6%E7%90%86%E3%81%AE%E9%A0%86%E7%95%AA
pub fn calculate_damage(
    p1: &PokeParam,
    p1_c: &PokeConst,
    m: &DamageMove,
    p2: &PokeParam,
    p2_c: &PokeConst,
) -> CalcDmgResult {
    let level = p1_c.level;
    let (atk, atk_rank, def, def_rank) = match m.kind {
        MoveKind::Physical => (p1.atk(), p1.atk_rank(), p2.def(), p2.def_rank()),
        MoveKind::Special => match m.unique_move {
            UniqueMove::サイコショック => {
                (p1.satk(), p1.satk_rank(), p2.def(), p2.def_rank())
            }
            _ => (p1.satk(), p1.satk_rank(), p2.sdef(), p2.sdef_rank()),
        },
        MoveKind::Status => panic!("Damage can't be calculated for status moves"),
    };
    let move_power = if m.damage_type == DamageType::VaryingPower {
        calc_varying_power(p1, p1_c, m, p2, p2_c)
    } else {
        m.power
    };
    //　急所ランク3以上は確定急所。ここでは確定してないものは考えない
    let critical = 3 <= p1.critical_rank() + m.critical as i32;

    let atk_type_boost = p1.def_types.contains(m.move_type);
    let teras_boost = if let Some(t) = p1.teras {
        t == m.move_type
    } else {
        false
    };
    let type_effectiveness = p2.def_types.effectiveness(m.move_type);

    let mut appliers: Vec<Applier> = vec![];

    add(&mut appliers, p1.ability.attacker(p1, m.kind, p2));
    add(&mut appliers, p2.ability.defender(p2, m.kind, p1));
    add(&mut appliers, p1.item.attacker(p1, m.kind, p2));
    add(&mut appliers, p2.item.defender(p2, m.kind, p1));

    let app = calc_appliers(&appliers);

    calculate_damage_main(
        level,
        move_power,
        atk,
        atk_rank,
        def,
        def_rank,
        critical,
        atk_type_boost,
        teras_boost,
        type_effectiveness,
        &app.power,
        &app.atk,
        &app.def,
        &app.damage,
    )
}

fn add(appliers: &mut Vec<Applier>, app: Option<Applier>) {
    if let Some(app) = app {
        appliers.push(app)
    }
}
