use std::collections::HashMap;

use crate::types::Types;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MoveKind {
    Physical,
    Special,
    Status,
}

#[derive(Debug)]
pub enum PokeMove {
    Damage(DamageMove),
    /// アイテムがないと威力が倍に
    アクロバット,
	/// 回復する
	ドレインパンチ,
	/// 体力を半分にする
	カタストロフィ,
	/// 先行で打つと飛行タイプがなくなる
    はねやすめ,
    /// 天気によってタイプが変わる
    ウェザーボール,
}

#[derive(Debug)]
pub struct DamageMove {
    pub id: isize,
    pub name: String,
    pub kind: MoveKind,
    pub move_type: Types,
    pub power: u32,
    /// そのうち使いたい
    pub contact: bool,
}

impl DamageMove {
    pub fn find_conflict(&self, m: &DamageMove) -> bool {
        if self.name == m.name
            && self.kind == m.kind
            && self.move_type == m.move_type
            && self.power == m.power
            && self.contact == m.contact
        {
            true
        } else {
            false
        }
    }
}

pub struct MoveStorage {
    pub map: HashMap<String, PokeMove>,
}

impl MoveStorage {
    pub fn add(&mut self, mut damage_move: DamageMove) -> Result<(), String> {
        if self.map.contains_key(&damage_move.name) {
            let PokeMove::Damage(item) = &self.map[&damage_move.name] else {
                Err(format!("The unique move '{}' already exists", damage_move.name))?
            };
            if item.find_conflict(&damage_move) {
                Err(format!("{} is conflicted", damage_move.name))?
            }
            Ok(())
        } else {
            let id = self.map.len();
            damage_move.id = id as isize;
            self.map
                .insert(damage_move.name.clone(), PokeMove::Damage(damage_move));
            Ok(())
        }
    }
}
