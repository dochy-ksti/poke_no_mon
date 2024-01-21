use rand::{rngs::ThreadRng, thread_rng, Rng};

use crate::{
    environment::environment::Environment,
    moves::damage_move::DamageMove,
    poke_params::{poke_const::PokeConst, poke_param::PokeParam},
    simulations::{calc_speed::calc_speed, calculate_damage::calculate_damage},
};

use super::playout_result::PlayoutResult;

pub fn battling(
    p1: &PokeParam,
    p1_c: &PokeConst,
    p1_move: &DamageMove,
    p2: &PokeParam,
    p2_c: &PokeConst,
    p2_move: &DamageMove,
) -> PlayoutResult {
    unimplemented!()
}

fn turn(
    turn: u32,
    p1: &PokeParam,
    p1_c: &PokeConst,
    p1_move: &DamageMove,
    p2: &PokeParam,
    p2_c: &PokeConst,
    p2_move: &DamageMove,
    env: &Environment,
) -> PlayoutResult {
    unimplemented!()
}

fn p1_plays_first(
    p1: &PokeParam,
    p1_c: &PokeConst,
    p1_move: &DamageMove,
    p2: &PokeParam,
    p2_c: &PokeConst,
    p2_move: &DamageMove,
    env: &Environment,
) -> bool {
    if p1_move.priority < p2_move.priority {
        return false;
    }
    if p2_move.priority < p1_move.priority {
        return true;
    }
    let p1_speed = calc_speed(p1, env);
    let p2_speed = calc_speed(p2, env);
    if p1_speed < p2_speed {
        return false;
    }
    if p2_speed < p1_speed {
        return true;
    }
    thread_rng().gen()
}

/// 基本的に、p1は自分、p2は相手、であるが、評価値を出すときにはこの「自分」がもともとP1だったかが問題になる。p1_is_p1で調べられるようにする
fn do_damage(
    turn: u32,
    p1_is_p1: bool,
    p1: &mut PokeParam,
    p1_c: &PokeConst,
    m: &DamageMove,
    p2: &mut PokeParam,
    p2_c: &PokeConst,
) -> Option<PlayoutResult> {
    let original_damage = calculate_damage(p1, p1_c, m, p2, p2_c);
    let damage = if p2.hp < original_damage.avg {
        p2.hp
    } else {
        original_damage.avg
    };
    p2.inflict_damage(damage);

    let drain = m.drain.calc4(damage);
    let recoil = m.recoil.calc4(damage);
    //ドレインと反動が一つの技で両方起きることは今のところないので、どういう順番で処理しても良いはず
    //もしかすると五捨五超入かもしれない。そうでないというソースが見つからない。
    p1.restore_hp(drain);
    p1.inflict_damage(recoil);

    p1.apply_rank_delta(&m.rank_delta);
    p2.apply_rank_delta(&m.oppo_rank_delta);

    if m.self_destruct == false {
        if p2.hp == 0 {
            if p1_is_p1 {
                return Some(PlayoutResult::p1_wins(turn, p1.hp));
            } else {
                return Some(PlayoutResult::p2_wins(turn, p1.hp));
            }
        }
        if p1.hp == 0 {
            if p1_is_p1 {
                return Some(PlayoutResult::p2_wins(turn, p2.hp));
            } else {
                return Some(PlayoutResult::p1_wins(turn, p2.hp));
            }
        }
    } else{
		if p1.hp == 0 {
            if p1_is_p1 {
                return Some(PlayoutResult::p2_wins(turn, p2.hp));
            } else {
                return Some(PlayoutResult::p1_wins(turn, p2.hp));
            }
        }
		if p2.hp == 0 {
            if p1_is_p1 {
                return Some(PlayoutResult::p1_wins(turn, p1.hp));
            } else {
                return Some(PlayoutResult::p2_wins(turn, p1.hp));
            }
        }
	}
    return None;
}
