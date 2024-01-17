use crate::{
    environment::environment::Environment,
    moves::unique_move::UniqueMove,
    poke_params::{poke_const::PokeConst, poke_param::PokeParam},
};

use super::appear::appear2;

pub fn game_start(
    mut p1: PokeParam,
    p1_c: &PokeConst,
    mut p2: PokeParam,
    p2_c: &PokeConst,
    mut env: Environment,
) {
    appear2(&mut p1, &mut p2, &mut env);
}
