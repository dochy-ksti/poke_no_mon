use std::collections::BTreeSet;

use crate::pnum::PNum;

use super::{
    applier::{Applier, ApplierType},
    atk_appliers::AtkAppliers,
    damage_appliers::DamageAppliers,
    def_appliers::DefAppliers,
    power_appliers::PowerAppliers, speed_appliers::SpeedAppliers,
};

pub fn calc_appliers(appliers: &[Applier]) -> AppliersResult {
    let mut power = BTreeSet::new();
    let mut atk = BTreeSet::new();
    let mut def = BTreeSet::new();
    let mut damage = BTreeSet::new();
    let mut speed = BTreeSet::new();

    for applier in appliers {
        match applier.t {
            ApplierType::Power => {
                power.insert(applier.num);
            }
            ApplierType::Atk => {
                atk.insert(applier.num);
            }
            ApplierType::Def => {
                def.insert(applier.num);
            }
            ApplierType::Damage => {
                damage.insert(applier.num);
            }
            ApplierType::Speed => {
                speed.insert(applier.num);
            }
        }
    }

    return AppliersResult {
        power: power
            .into_iter()
            .map(|n| PowerAppliers::from_u16(n).value())
            .collect(),
        atk: atk
            .into_iter()
            .map(|n| AtkAppliers::from_u16(n).value())
            .collect(),
        def: def
            .into_iter()
            .map(|n| DefAppliers::from_u16(n).value())
            .collect(),
        damage: damage
            .into_iter()
            .map(|n| DamageAppliers::from_u16(n).value())
            .collect(),
        speed: speed
            .into_iter()
            .map(|n| SpeedAppliers::from_u16(n).value())
            .collect(),
    };
}

pub struct AppliersResult {
    pub power: Vec<PNum>,
    pub atk: Vec<PNum>,
    pub def: Vec<PNum>,
    pub damage: Vec<PNum>,
    pub speed: Vec<PNum>,
}
