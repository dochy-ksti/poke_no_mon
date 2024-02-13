use super::damage_move::Options;

#[derive(Debug)]
pub enum Hits{
	Count(usize),
	Compound{ probability : usize, hit_max : usize, simplified_hits : usize, ikasama_hits : usize },
	Random{ hit_min : usize, hit_max : usize, simplified_hits : usize, ikasama_hits : usize },
}

impl Hits{
	pub fn new(o : &Options) -> Hits{

	}
}