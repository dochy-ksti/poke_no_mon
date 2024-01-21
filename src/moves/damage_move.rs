use std::{collections::HashMap, str::FromStr, sync::OnceLock};

use serde::Deserialize;

use crate::{
    pnum::PNum,
    poke_params::{ranks::Ranks, types::Types},
};

use super::{percent::Percent, unique_move::UniqueMove};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MoveKind {
    Physical,
    Special,
    Status,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum DamageType {
    Normal,
    ///威力の計算法が普通と違う
    VaryingPower,
    ///威力以外が普通と違う
    Unique,
    ///固定ダメージ
    Constant,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TestType {
    BattleTest,
    CompareNum,
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
    pub oppo_rank_delta: Ranks,
    pub drain: Percent,
    pub recoil: Percent,
    /// 0~3
    pub critical: u32,
	pub self_destruct : bool,
    pub test_type: TestType,
}

impl DamageMove {
    

    /// 使うたびに弱くなっていくような技は、ダメージが高くてもmost_damaging_moveとはみなせず、
    /// Drainする技や、反動がある技、防御が下がる技など、相手と殴り合わないと結果が見えない技もある。
    fn test_type(
        rank_delta: Ranks,
        oppo_rank_delta: Ranks,
        drain: Percent,
        recoil: Percent,
    ) -> TestType {
        if rank_delta == Ranks::default()
            && oppo_rank_delta == Ranks::default()
            && drain == Percent::V0
            && recoil == Percent::V0
        {
            TestType::CompareNum
        } else {
            TestType::BattleTest
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
    let move_type = Types::from_str(&move_type).expect(&format!("{name}: there's no type '{move_type}'"));
    let (damage_type, unique_move) = if u == "" {
        (DamageType::Normal, UniqueMove::NotUnique)
    } else if u == "U" {
        (DamageType::Unique, UniqueMove::from_str(&name).unwrap())
    } else if u == "V" {
        (
            DamageType::VaryingPower,
            UniqueMove::from_str(&name).unwrap(),
        )
    } else if u == "C" {
        (DamageType::Constant, UniqueMove::from_str(&name).unwrap())
    } else {
        panic!("{name}: the last arg '{u}' couldn't be recognized")
    };
	let priority = o.priority.unwrap_or(0);
	let rank_delta = o.rank();
	let oppo_rank_delta = o.oppo_rank();
	let drain = o.drain.map(|v| Percent::new(v)).unwrap_or(Percent::V0);
	let recoil = o.recoil.map(|v| Percent::new(v)).unwrap_or(Percent::V0);
	let critical = o.critical.unwrap_or(0);
	let self_destruct = o.self_destruct.is_some();
    DamageMove{
        name,
        kind,
        damage_type,
        unique_move,
        move_type,
        power,
        priority,
        rank_delta,
        oppo_rank_delta,
        drain,
        recoil, 
        critical,
		self_destruct,
		test_type: DamageMove::test_type(rank_delta, oppo_rank_delta, drain, recoil)
	}
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
    pub recoil: Option<u32>,
    pub critical: Option<u32>,
    pub atk: Option<i32>,
    pub def: Option<i32>,
    pub satk: Option<i32>,
    pub sdef: Option<i32>,
    pub speed: Option<i32>,
    pub oppo_atk: Option<i32>,
    pub oppo_def: Option<i32>,
    pub oppo_satk: Option<i32>,
    pub oppo_sdef: Option<i32>,
    pub oppo_speed: Option<i32>,
	pub self_destruct: Option<String>,
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

    fn oppo_rank(&self) -> Ranks {
        fn a(a: Option<i32>) -> i32 {
            a.unwrap_or(0)
        }
        Ranks::new(
            a(self.oppo_atk),
            a(self.oppo_def),
            a(self.oppo_satk),
            a(self.oppo_sdef),
            a(self.oppo_speed),
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
