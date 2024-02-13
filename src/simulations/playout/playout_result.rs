use super::{history::History, playout_eval::PlayoutEval};

pub struct PlayoutResult{
	pub eval : PlayoutEval,
	pub history : History,
}