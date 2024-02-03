use crate::{
    moves::{damage_move::TestType, poke_move::PokeMove},
    poke_params::{poke_const::PokeConst, poke_param::PokeParam}, simulations::find_most_damaging_move::find_most_damaging_move,
};

use super::battle_test::battling;



/// 変化技は使わず、確率で発生する追加効果は無視して、基本的には最大ダメージを与える技を探し、
/// どちらも一つの技だけを使ってどちらかが死ぬまで殴り続けるシミュレーションを行う。
/// がんじょうやマルチスケイルがあるため、ダメージの掛け算ではなく死ぬまでちゃんと殴る。
/// りゅうせいぐんやゴールドラッシュ、ドレインや反動技の評価が難しいため、追加効果がある技は最大ダメージ技でなくてもすべて試す。
/// やられてしまうことが分かったら、優先度が高い技を使ったら逆転できないかも調べる。
pub fn playout(p1: &PokeParam, p1_c: &PokeConst, p2: &PokeParam, p2_c: &PokeConst) -> PlayoutResult {
    let p1_move = find_most_damaging_move(p1, p1_c, p2, p2_c);
    let p2_move = find_most_damaging_move(p2, p2_c, p1, p1_c);

    for (p1_index, m) in p1_c.moves.iter().enumerate() {
        let PokeMove::Damage(p1_m) = m else {
            continue;
        };

        if p1_m.test_type == TestType::BattleTest || Some(p1_index) == p1_move {
            for (p2_index, m) in p2_c.moves.iter().enumerate() {
                let PokeMove::Damage(p2_m) = m else {
                    continue;
                };
				if p2_m.test_type == TestType::BattleTest || Some(p2_index) == p2_move{
					battling(p1, p1_c, p1_m, p2, p2_c, p2_m)
				}
            }
        }
        //if Some(index) == damaging_move_index {}
    }
}
