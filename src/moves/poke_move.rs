use super::{damage_move::DamageMove, unique_move::UniqueMove};

#[derive(Debug)]
pub enum PokeMove{
	Damage(DamageMove),
	Unique(UniqueMove)
}