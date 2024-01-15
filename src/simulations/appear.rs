use rand::{rngs::ThreadRng, Rng, thread_rng};

use crate::{environment::environment::Environment, poke_params::poke_param::PokeParam};

use super::{paradox_ability::paradox_ability, calc_speed::calc_speed};

// https://latest.pokewiki.net/%E3%83%90%E3%83%88%E3%83%AB%E4%B8%AD%E3%81%AE%E5%87%A6%E7%90%86%E3%81%AE%E9%A0%86%E7%95%AA
// https://wiki.xn--rckteqa2e.com/wiki/%E3%82%BF%E3%83%BC%E3%83%B3

/// 二体同時出現
pub fn appear2(p1: &mut PokeParam, p2: &mut PokeParam, env: &mut Environment) {
	appear_inner1(p1, p2, env, true);
	appear_inner2(p1, p2, env, true);
}

/// 1体出現
pub fn appear1(p1: &mut PokeParam, p2: &mut PokeParam, env: &mut Environment) {

    //let p1_speed = p1.speed();
    //let p2_speed = p2.speed();
}

/// speed補正値で実行する部分
fn appear_inner1(p1: &mut PokeParam, p2: &mut PokeParam, env: &mut Environment, is_appear2 : bool) {
	// let mut sp;
	// if is_appear2{
	// 	sp = Speeder::appear2(p1, p2, calc_speed(p1, env), calc_speed(p2, env), env)
	// } else{
	// 	//実際はappear1に変える必要あり
	// 	sp = Speeder::appear2(p1, p2, calc_speed(p1, env), calc_speed(p2, env), env)
	// }
	
    //かがくへんかガス等が先に発動する

    //いやしのねがい等の発動判定がある

    //まきびし、ねばねばネット等の判定がある

    //通常の優先度のとくせいがspeed順に発動する
    
}

/// speed実数値で実行する部分
fn appear_inner2(p1: &mut PokeParam, p2: &mut PokeParam, env: &mut Environment, is_appear2 : bool) {
	let mut sp;
	if is_appear2{
		sp = Speeder::appear2(p1, p2, p1.speed_raw(), p2.speed_raw(), env);
	} else{
		//実際はappear1に変える必要あり
		sp = Speeder::appear2(p1, p2,p1.speed_raw(), p2.speed_raw(), env);
	}
	sp.do1(paradox_ability);
}
pub struct Speeder<'a> {
    pub fast: &'a mut PokeParam,
    pub slow: &'a mut PokeParam,
    pub env: &'a mut Environment,
    pub pat: Pat,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Pat {
    //2体同時出現
    Appear2,
    //1体だけ出現
    Appear1,
    //2体同時出現かつ同速
    SameSpeed,
}

impl<'a> Speeder<'a> {
    pub fn appear1() {}
    pub fn appear2(
        p1: &'a mut PokeParam,
        p2: &'a mut PokeParam,
        speed1: u32,
        speed2: u32,
        env: &'a mut Environment,
    ) -> Self {
        // トリックルームの場合逆
        if speed1 == speed2 {
            Self {
                fast: p1,
                slow: p2,
                env,
                pat: Pat::SameSpeed,
            }
        } else if speed1 < speed2 {
            Self {
                fast: p2,
                slow: p1,
                env,
                pat: Pat::Appear2,
            }
        } else {
            Self {
                fast: p1,
                slow: p2,
                env,
                pat: Pat::Appear2,
            }
        }
    }

    pub fn do1(&mut self, f: fn(&mut PokeParam, &mut Environment)) {
        match self.pat {
            Pat::Appear2 => {
                f(self.fast, self.env);
                f(self.slow, self.env);
            }
            Pat::Appear1 => {
                f(self.fast, self.env);
            }
            Pat::SameSpeed => {
                let mut rng = thread_rng();
				if rng.gen(){
					f(self.fast, self.env);
                	f(self.slow, self.env);
				} else{
					f(self.fast, self.env);
                	f(self.slow, self.env);
				}
            }
        }
    }
}
