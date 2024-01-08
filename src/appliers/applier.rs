use super::{
    atk_appliers::AtkAppliers, damage_appliers::DamageAppliers, def_appliers::DefAppliers,
    power_appliers::PowerAppliers, speed_appliers::SpeedAppliers,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum ApplierType {
    Power,
    Atk,
    Def,
    Damage,
    Speed,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Applier {
    pub t: ApplierType,
    pub num: u16,
}

impl Applier {
    pub fn power(app: PowerAppliers) -> Applier {
        todo!("applier にも temporary（ターン数で終わる) permanent(永続) interim(交代まで続く)の３つがあるはず。");
        Applier {
            t: ApplierType::Power,
            num: app as u16,
        }
    }
    pub fn atk(app: AtkAppliers) -> Applier {
        Applier {
            t: ApplierType::Atk,
            num: app as u16,
        }
    }
    pub fn def(app: DefAppliers) -> Applier {
        Applier {
            t: ApplierType::Def,
            num: app as u16,
        }
    }
    pub fn damage(app: DamageAppliers) -> Applier {
        Applier {
            t: ApplierType::Damage,
            num: app as u16,
        }
    }
    pub fn speed(app: SpeedAppliers) -> Applier {
        Applier {
            t: ApplierType::Speed,
            num: app as u16,
        }
    }
}
