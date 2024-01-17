use crate::{
    moves::{
        damage_move::{DamageMove, UniqueType},
        poke_move::PokeMove,
    },
    poke_params::{poke_const::PokeConst, poke_param::PokeParam},
};

use super::calculate_damage::calculate_damage;

///追加効果とかは完全に無視してダメージだけを見る。これを使ってプレイアウトすると、
/// りゅうせいぐんやドレインパンチのような技は正しく評価できないが、まあ別に良いのではないか。
/// ある程度全探索していれば、最後の評価が適当でもちゃんと機能するはずだ。5ターンぐらいは読めると思うし。
pub fn find_most_damaging_move(p1: &PokeParam, p1_c: &PokeConst, p2: &PokeParam) -> Option<usize> {
    let mut damage: isize = -1;
    let mut index: isize = -1;
    for (i, poke_move) in p1_c.moves.iter().enumerate() {
        if let PokeMove::Damage(m) = poke_move {
            let power = match m.unique_type {
                UniqueType::NotUnique => m.power,
                UniqueType::UniquePower => calc_unique_power(m),
                UniqueType::Constant => {
                    let d = calc_constant_damage(m);
                    update(&mut damage, &mut index, d, i);
                    continue;
                }
            };

            let r = calculate_damage(p1, p1_c, power, m, p2);
            update(&mut damage, &mut index, r.avg, i);
        }
    }
    if index < 0 {
        return None;
    } else {
        return Some(index as usize);
    }
}

fn update(damage: &mut isize, index: &mut isize, new_damage: u32, new_index: usize) {
    if *damage < new_damage as isize {
        *damage = new_damage as isize;
        *index = new_index as isize;
    }
}

fn calc_unique_power(damage: &DamageMove) -> u32 {
    unimplemented!()
}

fn calc_constant_damage(damage: &DamageMove) -> u32 {
    unimplemented!()
}
