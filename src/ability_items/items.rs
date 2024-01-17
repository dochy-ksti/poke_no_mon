use strum_macros::EnumString;

use crate::{pnum::PNum, poke_params::poke_param::PokeParam, moves::damage_move::MoveKind, appliers::{applier::Applier, power_appliers::PowerAppliers, atk_appliers::AtkAppliers, def_appliers::DefAppliers, damage_appliers::DamageAppliers}};

#[derive(Debug, PartialEq, Clone, Copy, EnumString)]
pub enum Items{
	なし,
	// 威力補正1.1
	//パンチグローブ等,
	// 威力補正1.2
	//もくたん等,

	/// ダメージ補正1.3 5325
	いのちのたま,

	/// 攻撃補正1.5 6144
	こだわりハチマキ,
	/// 特攻補正1.5 6144
	こだわりメガネ,

	/// 特防補正1.5 6144
	とつげきチョッキ,

	
	ブーストエナジー,

	こだわりスカーフ
}

impl Items{
    pub fn defender(&self, p1: &PokeParam, move_kind: MoveKind, p2: &PokeParam) -> Option<Applier> {
        use Items as I;

        match self {
            I::とつげきチョッキ => {
				if move_kind == MoveKind::Special{
					return def(DefAppliers::とつげきチョッキ等);
				}
            }
            _ =>{}
        }
		None
    }

    pub fn attacker(&self, p1 : &PokeParam, move_kind: MoveKind, p2 : &PokeParam) -> Option<Applier> {
        use Items as I;

        match self {
            I::こだわりハチマキ => {
                if move_kind == MoveKind::Physical {
                    return atk(AtkAppliers::こだわりハチマキ等);
                } 
            }
            I::こだわりメガネ => {
                if move_kind == MoveKind::Special {
                    return atk(AtkAppliers::こだわりハチマキ等);
                } 
            }
            _ =>{}
        }
		return None;
    }
}

fn power(app: PowerAppliers) -> Option<Applier> {
    Some(Applier::power(app))
}

fn atk(app: AtkAppliers) -> Option<Applier> {
    Some(Applier::atk(app))
}

fn def(app: DefAppliers) -> Option<Applier> {
    Some(Applier::def(app))
}

fn damage(app: DamageAppliers) -> Option<Applier> {
    Some(Applier::damage(app))
}
