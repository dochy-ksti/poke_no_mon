use super::damage_move::Options;

#[derive(Debug)]
pub enum Hits{
	Count(usize),
	Compound{ probability : usize, hit_max : usize },
	Random{ hit_min : usize, hit_max : usize },
}

impl Hits{
	pub fn new(o : &Options) -> Hits{
		
	}
}