use crate::{poke_params::poke_param::PokeParam, environment::environment::Environment};

use super::{speeder::Speeder, appear::appear};

pub fn game_start(p1 : PokeParam, p2 : PokeParam, mut env : Environment){
	let mut sp = Speeder::new(p1, Some(p2));
	appear(&mut sp, &mut env)
}