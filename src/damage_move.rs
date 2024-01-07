use std::{collections::HashMap, str::FromStr, sync::OnceLock};

use serde::Deserialize;

use crate::{pnum::PNum, types::Types};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MoveKind {
    Physical,
    Special,
    Status,
}

#[derive(Debug, Clone)]
pub struct DamageMove {
    pub name: String,
    pub kind: MoveKind,
    pub is_unique: bool,
    pub move_type: Types,
    pub power: u32,
    /// そのうち使いたい
    //pub contact: bool,
    pub priority: i32,
    pub drain: Option<PNum>,
}

impl DamageMove {
    pub fn new(
        name: String,
        kind: MoveKind,
        is_unique: bool,
        move_type: Types,
        power: u32,
        priority: i32,
        drain: Option<PNum>,
    ) -> Self {
        Self {
            name,
            kind,
            is_unique,
            move_type,
            power,
            priority,
            drain,
        }
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
    let is_unique = if u == "" {
        false
    } else if u == "U" {
        true
    } else {
        panic!("{name}: last arg '{u}' can't be recognized")
    };
    DamageMove::new(
        name,
        kind,
        is_unique,
        t,
        power,
        o.priority.unwrap_or(0),
        o.drain.map(|v| PNum::from_percent(v)),
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
