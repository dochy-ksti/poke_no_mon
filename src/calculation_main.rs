use crate::pnum::PNum;

pub struct CalcMainResult {
    pub min: u32,
    pub max: u32,
    pub avg: u32,
}

pub fn calculation_main(
    level: u32,
    move_power: u32,
    atk: u32,
    def: u32,
    atk_rank: i32,
    def_rank: i32,
    atk_type_boost: bool,
    teras_boost: bool,
    type_effectiveness: PNum,
    power_appliers: &[PNum],
    atk_appliers: &[PNum],
    def_appliers: &[PNum],
    damage_appliers: &[PNum],
) -> CalcMainResult {
    let power = calc_power(move_power, power_appliers, teras_boost);
    let atk = calc_atk(atk, atk_rank, atk_appliers);
    let def = calc_def(def, def_rank, def_appliers);
    let d = (level * 2) / 5 + 2;
    let d = (d * power * atk) / def;
    let mut d = d / 50 + 2;
    let min = d * 85 / 100;
    let max = d;
    let avg = (d * 925) / 1000; //乱数の平均値0.925

    let damage_applier = calc_applier(damage_appliers);
    CalcMainResult {
        min: rest(min, atk_type_boost, type_effectiveness, damage_applier),
        avg: rest(avg, atk_type_boost, type_effectiveness, damage_applier),
        max: rest(max, atk_type_boost, type_effectiveness, damage_applier),
    }
}

fn rest(mut d: u32, atk_type_boost: bool, type_effectiveness: PNum, damage_applier: PNum) -> u32 {
    if atk_type_boost {
        d = PNum::V1_5.apply5(d);
    }
    let d = type_effectiveness.apply(d);
    let d = damage_applier.apply5(d);
    if type_effectiveness.val != 0 {
        if d == 0 {
            1
        } else {
            d
        }
    } else {
        d
    }
}

fn calc_power(move_power: u32, power_appliers: &[PNum], teras_boost: bool) -> u32 {
    let mut power = calc_applier(power_appliers).apply5(move_power);
    if power == 0 {
        power = 1;
    }
    if teras_boost && power < 60 {
        power = 60;
    }
    power
}

fn calc_atk(atk: u32, atk_rank: i32, atk_appliers: &[PNum]) -> u32 {
    let atk = calc_rank(atk, atk_rank);
    let atk = calc_applier(atk_appliers).apply5(atk);
    if atk == 0 {
        1
    } else {
        atk
    }
}

fn calc_def(def: u32, def_rank: i32, def_appliers: &[PNum]) -> u32 {
    let def = calc_rank(def, def_rank);
    let def = calc_applier(def_appliers).apply5(def);
    //本来天候の計算が入るが今はまだ使ってない
    if def == 0 {
        1
    } else {
        def
    }
}

fn calc_applier(appliers: &[PNum]) -> PNum {
    let mut v = PNum::V1;
    for applier in appliers {
        v = v.apply4(*applier);
    }
    v
}

fn calc_rank(value: u32, rank: i32) -> u32 {
    if 0 <= rank {
        (value * (2 + rank as u32)) / 2
    } else {
        (value * 2) / (2 + (-rank) as u32)
    }
}
