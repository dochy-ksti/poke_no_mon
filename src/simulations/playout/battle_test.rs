use crate::{
    moves::damage_move::DamageMove,
    poke_params::{poke_const::PokeConst, poke_param::PokeParam}, simulations::calc_speed::calc_speed,
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
	turn : usize,
	p1: &PokeParam,
    p1_c: &PokeConst,
	p1_move: &DamageMove,
    p2: &PokeParam,
    p2_c: &PokeConst,
    p2_move: &DamageMove,) -> PlayoutResult{
		
		unimplemented!()
}

fn p1_plays_first(p1: &PokeParam,
    p1_c: &PokeConst,
	p1_move: &DamageMove,
    p2: &PokeParam,
    p2_c: &PokeConst,
    p2_move: &DamageMove) -> bool
{
	if p1_move.priority < p2_move.priority{
		return false;
	} 
	if p2_move.priority < p1_move.priority{
		return true;
	}
	let p1_speed = calc_speed(p, env)
}

fn do_damage(p1 : &PokeParam, p1_move : &DamageMove, p2 : &PokeParam){
	
}
