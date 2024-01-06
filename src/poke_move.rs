use std::collections::HashMap;

use crate::{types::Types, pnum::PNum};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MoveKind {
    Physical,
    Special,
    Status,
}

#[derive(Debug)]
pub enum PokeMove {
	B 炎 100 ツタコンボウ 
	B 草 75 ウッドホーン
	SB 無 40 でんこうせっか 
	B 格 120 ばかぢから
	B 悪 80 じごくづき
	B 草 120 パワーウィップ
	B 岩 60 がんせきふうじ
	B 超 80 しねんのずつき
	B 虫 70 とんぼがえり
	B 地 75 じだんだ
	B 妖 90 じゃれつく
    /// アイテムがないと威力が倍に
    アクロバット,
	/// 半分を回復する
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
	pub priority : i32,
	pub drain : PNum,
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
