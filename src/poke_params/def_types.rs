use crate::pnum::PNum;

use super::types::Types;


#[derive(Debug, Clone, Copy)]
/// ポケモンはひとつまたはふたつのタイプを持つ
pub struct DefTypes{
	pub type1 : Types,
	pub type2 : Option<Types>,
}

impl DefTypes{
	pub fn effectiveness(&self, move_type : Types) -> PNum{
		let first = move_type.effectiveness(self.type1);
		let Some(t2) = self.type2 else{
			return first
		};
		let second = move_type.effectiveness(t2);
		first.mul(second)
	}
	pub fn contains(&self, move_type : Types) -> bool{
		if self.type1 == move_type{
			return true;
		} 
		if let Some(type2) = self.type2{
			if type2 == move_type{
				return true;
			}
		}
		false
	}
}