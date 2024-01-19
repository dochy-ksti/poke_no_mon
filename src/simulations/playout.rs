use crate::poke_params::{poke_const::PokeConst, poke_param::PokeParam};

use super::find_most_damaging_move::find_most_damaging_move;

pub struct PlayoutResult {
    turns: u32,
}

/// 変化技は使わず、確率で発生する追加効果は無視して、基本的には最大ダメージを与える技を探し、
/// 同じ技を使って死ぬまで殴り続けるシミュレーションを行う。
/// がんじょうやマルチスケイルがあるため、ダメージの掛け算ではなく死ぬまでちゃんと殴る。
/// りゅうせいぐんやゴールドラッシュの評価が難しいため、追加効果がある技は最大ダメージ技とは別で試し、
/// 結果が一番良いものを採用する。
/// スケイルショットのように防御が下がる技はさらに難しいが、これはもうわからないので考慮しない
pub fn playout(p1: &PokeParam, p1_c: &PokeConst, p2: &PokeParam) {
    let damaging_move_index = find_most_damaging_move(p1, p1_c, p2);
        

    for (index, m) in p1_c.moves.iter().enumerate() {
        if index == damaging_move_index {}
    }
}
