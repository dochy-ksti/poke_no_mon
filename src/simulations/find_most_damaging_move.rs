use crate::{poke_params::{poke_param::PokeParam, poke_const::PokeConst}, moves::{poke_move::PokeMove, damage_move::{UniqueType, DamageMove}}};


///追加効果とかは完全に無視してダメージだけを見る。これを使ってプレイアウトすると、
/// りゅうせいぐんやドレインパンチのような技は正しく評価できないが、まあ別に良いのではないか。
/// ある程度全探索していれば、最後の評価が適当でもちゃんと機能するはずだ。5ターンぐらいは読めると思うし。
pub fn find_most_damaging_move(p1 : &PokeParam, p1_c : &PokeConst, p2 : &PokeParam){
	let mut damage : isize = -1;
	let mut index : isize = -1;
	for (i, poke_move) in p1_c.moves.iter().enumerate(){
		if let PokeMove::Damage(m) = poke_move{
			let power = match m.unique_type{
				UniqueType::NotUnique =>{ m.power }
				UniqueType::UniquePower =>{ calc_unique_power(m) }
				UniqueType::Constant =>{
					let d = calc_constant_damage(m);
					update(&mut damage, &mut index, d, i);
					continue;
				}
			};

			
		} 
	}
}

fn update(damage : &mut isize, index : &mut isize, new_damage : u32, new_index : usize){
	if *damage < new_damage as isize{
		*damage = new_damage as isize;
		*index = new_index as isize;
	}
}

fn calc_unique_power(damage : &DamageMove) -> u32{
	unimplemented!()
}

fn calc_constant_damage(damage : &DamageMove) -> u32{
	unimplemented!()
}

