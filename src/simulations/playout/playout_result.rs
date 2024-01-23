/// 大きいほどP1にとって良い結果であり、
/// 小さいほどP2にとって良い結果である
#[derive(Debug, PartialEq)]
pub struct PlayoutResult {
    // p1が勝つと i16::MAX - 決着までのターン数
    // p2が勝つと i16::MIN + 決着までのターン数
    // 規定ターン数以内に勝負がつかない場合0になる。
    pub turns: i32,
    // 勝者の残りHP。p1が勝つとプラス、p2だとマイナス。
    pub rest_hp: i32,
    // p1_hp/p1_max_hp / p2_hp/p2_max_hp
    // p2_max_hp*p1_hp / p1_max_hp * p2_hp
    // なので、p2が有利なほど小さく、p1が有利なほど大きくなる
    // 決着がつかなかった場合、つまりturns == 0の場合だけ値を持ち、それ以外は0
    pub hp_rate: f32,
}

impl Eq for PlayoutResult {}

impl PartialOrd for PlayoutResult {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.turns.partial_cmp(&other.turns) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        if self.turns == 0 {
            self.hp_rate.partial_cmp(&other.hp_rate)
        } else {
            self.rest_hp.partial_cmp(&other.rest_hp)
        }
    }
}

impl Ord for PlayoutResult {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PlayoutResult {
    pub fn p1_wins(turn: u32, rest_hp: u32) -> PlayoutResult {
        let turns = i16::MAX as i32 - turn as i32;
        PlayoutResult {
            turns,
            rest_hp: rest_hp as i32,
            hp_rate: 0.0,
        }
    }
    pub fn p2_wins(turn: u32, rest_hp: u32) -> PlayoutResult {
        let turns = i16::MIN as i32 + turn as i32;
        PlayoutResult {
            turns,
            rest_hp: rest_hp as i32 * -1,
            hp_rate: 0.0,
        }
    }
    pub fn not_end(
        turn: u32,
        p1_max_hp: u32,
        p1_hp: u32,
        p2_max_hp: u32,
        p2_hp: u32,
    ) -> PlayoutResult {
        if p2_hp == 0 {
            if p1_hp == 0 {
                return PlayoutResult {
                    turns: 0,
                    rest_hp: 0,
                    hp_rate: 1.0,
                };
            } else {
                return Self::p1_wins(turn, p1_hp);
            }
        }
        if p1_hp == 0 {
            return Self::p2_wins(turn, p2_hp);
        }
        let hp_rate = (p2_max_hp * p1_hp) as f32 / (p1_max_hp * p2_hp) as f32;
        PlayoutResult {
            turns: 0,
            rest_hp: 0,
            hp_rate,
        }
    }

	pub fn both_dead() -> PlayoutResult{
		PlayoutResult{ turns: 0, rest_hp : 0, hp_rate: 0.0}
	}
}
