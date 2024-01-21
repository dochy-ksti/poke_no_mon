
/// 大きいほどP1にとって良い結果であり、
/// 小さいほどP2にとって良い結果である
#[derive(Debug, PartialEq)]
pub struct PlayoutResult {
	// p1が勝つと i16::MAX - 決着までのターン数
	// p2が勝つと i16::MIN + 決着までのターン数
	// 規定ターン数以内に勝負がつかない場合0になる。
    pub turns: i32,
	// 勝者の残りHP。p1が勝つとプラス、p2だとマイナス。
	pub rest_hp : i32,
	// p1_hp/p1_max_hp / p2_hp/p2_max_hp
	// p2_max_hp*p1_hp / p1_max_hp * p2_hp
	// なので、p2が有利なほど小さく、p1が有利なほど大きくなる
	// 決着がつかなかった場合、つまりturns == 0の場合だけ値を持ち、それ以外は0
	pub hp_rate: f32,
}

impl PartialOrd for PlayoutResult{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.turns.partial_cmp(&other.turns) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
		if self.turns == 0{
        	self.hp_rate.partial_cmp(&other.hp_rate)
		} else {
			self.rest_hp.partial_cmp(&other.rest_hp)
		} 
    }
}

impl PlayoutResult{
	pub fn p1_wins(turn : u32, rest_hp : u32) -> PlayoutResult{
		
	}
}