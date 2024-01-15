use crate::{
    environment::environment::Environment, moves::unique_move::PokeMove,
    poke_params::poke_param::PokeParam,
};

use super::appear::appear2;

pub fn game_start(
    mut p1: PokeParam,
    p1_moves: &[PokeMove],
    mut p2: PokeParam,
    p2_moves: &[PokeMove],
    mut env: Environment,
) {
    appear2(&mut p1, &mut p2, &mut env);
}
