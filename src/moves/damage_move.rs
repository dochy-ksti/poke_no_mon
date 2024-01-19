use std::{collections::HashMap, str::FromStr, sync::OnceLock};

use serde::Deserialize;

use crate::{
    pnum::PNum,
    poke_params::{ranks::Ranks, types::Types},
};

use super::unique_move::UniqueMove;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MoveKind {
    Physical,
    Special,
    Status,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DamageType {
    Normal,
    ///威力の計算法が特殊
    VaryingPower,
    ///固定ダメージ
    Constant,
}

#[derive(Debug)]
pub struct DamageMove {
    pub name: String,
    pub kind: MoveKind,
    pub damage_type: DamageType,
    pub unique_move: UniqueMove,
    pub move_type: Types,
    pub power: u32,
    /// そのうち使いたい
    //pub contact: bool,
    pub priority: i32,
    pub rank_delta: Ranks,
    pub drain: PNum,
    pub is_normal: bool,
}

impl DamageMove {
    pub fn new(
        name: String,
        kind: MoveKind,
        unique_type: DamageType,
        unique_move: UniqueMove,
        move_type: Types,
        power: u32,
        priority: i32,
        rank_delta: Ranks,
        drain: PNum,
    ) -> Self {
        Self {
            name,
            kind,
            damage_type: unique_type,
            unique_move,
            move_type,
            power,
            priority,
            rank_delta,
            drain,
            is_normal: Self::is_normal(priority, rank_delta, drain),
        }
    }

    /// 使うたびに弱くなっていくような技は、ダメージが高くてもmost_damaging_moveとはみなせず、
    /// 非normal_moveと考えて別枠で処理する必要がある。その場合 is_normal でfalseを返す
    fn is_normal(priority: i32, rank_delta: Ranks, drain: PNum) -> bool {
        priority == 0 && rank_delta == Ranks::default() && drain == PNum::V1
    }
}

fn create_damage(
    name: String,
    kind: MoveKind,
    move_type: String,
    power: u32,
    u: String,
    o: Options,
) -> DamageMove {
    let t = Types::from_str(&move_type).expect(&format!("{name}: there's no type '{move_type}'"));
    let (unique_type, unique_move) = if u == "" {
        (DamageType::Normal, UniqueMove::NotUnique)
    } else if u == "U" {
        (
            DamageType::VaryingPower,
            UniqueMove::from_str(&name).unwrap(),
        )
    } else if u == "C" {
        (DamageType::Constant, UniqueMove::from_str(&name).unwrap())
    } else {
        panic!("{name}: last arg '{u}' can't be recognized")
    };
    DamageMove::new(
        name,
        kind,
        unique_type,
        unique_move,
        t,
        power,
        o.priority.unwrap_or(0),
		o.rank(),
        o.drain.map(|v| PNum::from_percent(v)).unwrap_or(PNum::V0),
    )
}

#[derive(Debug, Deserialize)]
pub enum DamageMoveSyntax {
    T(String, u32, String, String, Options),
    B(String, u32, String, String, Options),
}

#[derive(Debug, Deserialize)]
pub struct Options {
    pub priority: Option<i32>,
    pub drain: Option<u32>,
    pub atk: Option<i32>,
    pub def: Option<i32>,
    pub satk: Option<i32>,
    pub sdef: Option<i32>,
    pub speed: Option<i32>,
}

impl Options {
    fn rank(&self) -> Ranks {
        fn a(a: Option<i32>) -> i32 {
            a.unwrap_or(0)
        }
        Ranks::new(
            a(self.atk),
            a(self.def),
            a(self.satk),
            a(self.sdef),
            a(self.speed),
        )
    }
}

impl DamageMoveSyntax {
    pub fn to_damage(self) -> DamageMove {
        match self {
            DamageMoveSyntax::T(t, p, n, u, o) => create_damage(n, MoveKind::Special, t, p, u, o),
            DamageMoveSyntax::B(t, p, n, u, o) => create_damage(n, MoveKind::Physical, t, p, u, o),
        }
    }
}

static DAMAGE_MOVE_STORAGE: OnceLock<HashMap<String, DamageMove>> = OnceLock::new();

pub fn damage_move_storage() -> &'static HashMap<String, DamageMove> {
    &DAMAGE_MOVE_STORAGE.get_or_init(|| {
        let vec: Vec<DamageMoveSyntax> = munyo::from_file("moves/damage_moves.txt").unwrap();
        let mut map: HashMap<String, DamageMove> = HashMap::with_capacity(vec.len());
        for item in vec {
            let damage = item.to_damage();
            map.insert(damage.name.to_string(), damage);
        }
        map
    })
}
