use crate::{
    appliers::{
        applier::Applier, atk_appliers::AtkAppliers, damage_appliers::DamageAppliers,
        def_appliers::DefAppliers, power_appliers::PowerAppliers,
    },
    moves::damage_move::MoveKind,
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

    こだいかっせい,
    クオークチャージ,
}

impl Abilities {
    pub fn defender(&self, move_kind: MoveKind, def_max_hp: u32, def_hp: u32) -> Option<Applier> {
        use Abilities as A;

        match self {
            A::マルチスケイル => {
                if def_hp == def_max_hp {
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
            _ => None,
        }
    }

    pub fn attacker(&self, move_kind: MoveKind) -> Option<Applier> {
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
