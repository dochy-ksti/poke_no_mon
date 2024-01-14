use crate::poke_params::poke_param::PokeParam;

pub struct Speeder{
	pub fast : PokeParam,
	pub slow : Option<PokeParam>
}

impl Speeder{
	pub fn new(p1 : PokeParam, p2 : Option<PokeParam>) -> Speeder{
		if let Some(p2) = p2{
			if p1.speed() < p2.speed(){
				Speeder{ fast : p2, slow : Some(p1) }
			} else{
				Speeder{ fast : p1, slow : Some(p2)}
			}
		} else{
			Speeder{ fast : p1, slow : None }
		}
	}

	pub fn new_raw(p1 : PokeParam, p2 : Option<PokeParam>) -> Speeder{
		if let Some(p2) = p2{
			if p1.speed() < p2.speed(){
				Speeder{ fast : p2, slow : Some(p1) }
			} else{
				Speeder{ fast : p1, slow : Some(p2)}
			}
		} else{
			Speeder{ fast : p1, slow : None }
		}
	}

	///スピードが早い方を実行し、その後あれば遅い方を実行する
	pub fn process<F>(&mut self, mut f : F) where F : FnMut(&mut PokeParam){
		f(&mut self.fast);
		self.slow.as_mut().map(|p| f(p));
	}
}