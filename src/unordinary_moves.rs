use std::str::FromStr;
use strum_macros::EnumString;

#[derive(Debug, PartialEq, Eq, Clone, Copy, EnumString)]
pub enum UnordinaryMoves{
	ドレインパンチ,
	カタストロフィ,
}

impl UnordinaryMoves{
	fn from_str(s : &str) -> Option<UnordinaryMoves>{
		FromStr::from_str(s).ok()
	}
}