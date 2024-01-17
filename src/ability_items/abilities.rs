use crate::{
    appliers::{
        applier::Applier, atk_appliers::AtkAppliers, damage_appliers::DamageAppliers,
        def_appliers::DefAppliers, power_appliers::PowerAppliers,
    },
    moves::damage_move::MoveKind,
    poke_params::poke_param::{ParadoxBoost, PokeParam},
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Abilities {
    // 1.3
    //ちからずく等,
    // 1.2
    //てつのこぶし等,

    // 攻撃補正2.0 8192
    //ちからもち等,
    わざわいのたま,
    わざわいのつるぎ,
    わざわいのうつわ,
    わざわいのおふだ,

    // 防御補正 体力最大値のとき0.5
    マルチスケイル,

    ノーてんき,

	かたやぶり,

    こだいかっせい,
    クオークチャージ,
}

impl Abilities {
    pub fn defender(&self, p1: &PokeParam, move_kind: MoveKind, p2: &PokeParam) -> Option<Applier> {
        use Abilities as A;

        match self {
            A::マルチスケイル => {
                if p1.hp == p1.stats.hp() && p2.ability != Abilities::かたやぶり{
                    damage(DamageAppliers::マルチスケイル)
                } else {
                    None
                }
            }
            A::わざわいのおふだ => {
                if move_kind == MoveKind::Physical {
                    atk(AtkAppliers::わざわい器札)
                } else {
                    None
                }
            }
            A::わざわいのうつわ => {
                if move_kind == MoveKind::Special {
                    atk(AtkAppliers::わざわい器札)
                } else {
                    None
                }
            }
            A::クオークチャージ | A::こだいかっせい => {
                match p1.paradox_boost() {
                    ParadoxBoost::Def => {
                        if move_kind == MoveKind::Physical {
                            return def(DefAppliers::クオークチャージ等);
                        }
                    }
                    ParadoxBoost::SDef => {
                        if move_kind == MoveKind::Special {
                            return def(DefAppliers::クオークチャージ等);
                        }
                    }
                    _ => {}
                }
                return None;
            }
            _ => None,
        }
    }

    pub fn attacker(&self, p1 : &PokeParam, move_kind: MoveKind, p2 : &PokeParam) -> Option<Applier> {
        use Abilities as A;

        match self {
            A::わざわいのたま => {
                if move_kind == MoveKind::Special {
                    def(DefAppliers::わざわい玉剣)
                } else {
                    None
                }
            }
            A::わざわいのつるぎ => {
                if move_kind == MoveKind::Physical {
                    def(DefAppliers::わざわい玉剣)
                } else {
                    None
                }
            }
			A::クオークチャージ | A::こだいかっせい =>{
				match p1.paradox_boost(){
					ParadoxBoost::Atk =>{
						if move_kind == MoveKind::Physical{
							return atk(AtkAppliers::クオークチャージ攻);
						}
					}
					ParadoxBoost::SAtk =>{
						if move_kind == MoveKind::Special{
							return atk(AtkAppliers::クオークチャージ攻);
						}
					}
					_ =>{}
				}
				return None;
			}
            _ => None,
        }
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
