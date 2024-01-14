use crate::{poke_params::poke_param::PokeParam, environment::environment::Environment};

use super::{paradox_ability::paradox_ability, speeder::Speeder};

// https://latest.pokewiki.net/%E3%83%90%E3%83%88%E3%83%AB%E4%B8%AD%E3%81%AE%E5%87%A6%E7%90%86%E3%81%AE%E9%A0%86%E7%95%AA
// https://wiki.xn--rckteqa2e.com/wiki/%E3%82%BF%E3%83%BC%E3%83%B3

pub fn appear(sp : &mut Speeder, env : &mut Environment){

	//let p1_speed = p1.speed();
	//let p2_speed = p2.speed();
	//かがくへんかガス等が先に発動する

	//いやしのねがい等の発動判定がある

	//まきびし、ねばねばネット等の判定がある

	//通常の優先度のとくせいがspeed順に発動する

	//paradoxは通常の特性より優先度が低いが、異様なほど発動条件が複雑。
	//まきびしでダメージを受けたときでも、天候が晴れなら即座にこだいかっせいは発動し、
	//ねばねばネットがその後にあったとしても、その効果は無視されるはずである。ホンマか？
	//https://wiki.xn--rckteqa2e.com/wiki/%E3%82%BF%E3%83%BC%E3%83%B3
	sp.process(|p| paradox_ability(p,env));
	
}