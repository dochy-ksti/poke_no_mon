/// 大きいほどP1にとって良い結果であり、
/// 小さいほどP2にとって良い結果である
pub struct PlayoutEval {
    /// p1が勝つと i16::MAX - 決着までのターン数
    /// p2が勝つと i16::MIN + 決着までのターン数
    /// 規定ターン数以内に勝負がつかない場合0になる。
    pub turns: i32,

    u: Union,
}

/// turns が 0 のときは hp_rate, それ以外は rest_hp として振る舞う
union Union {
    // 勝者の残りHP。p1が勝つとプラス、p2だとマイナス。
    rest_hp: i32,
    // p1_hp/p1_max_hp / p2_hp/p2_max_hp
    // p2_max_hp*p1_hp / p1_max_hp * p2_hp
    // なので、p2が有利なほど小さく、p1が有利なほど大きくなる
    hp_rate: f32,
}

impl PartialEq for PlayoutEval {
    fn eq(&self, other: &Self) -> bool {
        unsafe { compare(self, other).is_eq() }
    }
}

impl Eq for PlayoutEval {}

impl PartialOrd for PlayoutEval {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(compare(self, other))
    }
}

impl Ord for PlayoutEval {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        compare(self, other)
    }
}

fn compare(l : &PlayoutEval, r : &PlayoutEval) -> std::cmp::Ordering{
	if l.turns != r.turns{
		return l.turns.cmp(&r.turns);
	}
	if l.turns == 0 {
		unsafe { l.u.hp_rate.partial_cmp(&r.u.hp_rate).unwrap() }
	} else {
		unsafe { l.u.rest_hp.cmp(&r.u.rest_hp) }
	}
}

impl PlayoutEval {
    /// p1が勝つと 0 < turns
    /// p2が勝つと turns < 0
    /// 決着がついてない場合 0
    pub fn turns(&self) -> i32 {
        if 0 < self.turns {
            i16::MAX as i32 - self.turns
        } else if self.turns < 0 {
            i16::MIN as i32 - self.turns
        } else {
            0
        }
    }
    /// 勝者の残りHP。p1が勝つとプラス、p2だとマイナス。
    /// turns == 0 のときは None
    pub fn rest_hp(&self) -> Option<i32> {
        if self.turns != 0 {
            Some(unsafe { self.u.rest_hp })
        } else {
            None
        }
    }

    /// p1_hp/p1_max_hp / p2_hp/p2_max_hp
    /// p2_max_hp*p1_hp / p1_max_hp * p2_hp
    /// なので、常に正である。p2が有利なほど小さく、p1が有利なほど大きくなる。
    /// turns != 0のときは None
    pub fn hp_rate(&self) -> Option<f32> {
        if self.turns == 0 {
            Some(unsafe { self.u.hp_rate })
        } else {
            None
        }
    }

    pub fn p1_wins(turn: u32, rest_hp: u32) -> PlayoutEval {
        let turns = i16::MAX as i32 - turn as i32;
        PlayoutEval {
            turns,
            u: Union {
                rest_hp: rest_hp as i32,
            },
        }
    }
    pub fn p2_wins(turn: u32, rest_hp: u32) -> PlayoutEval {
        let turns = i16::MIN as i32 + turn as i32;
        PlayoutEval {
            turns,
            u: Union {
                rest_hp: rest_hp as i32 * -1,
            },
        }
    }
    pub fn not_end(
        turn: u32,
        p1_max_hp: u32,
        p1_hp: u32,
        p2_max_hp: u32,
        p2_hp: u32,
    ) -> PlayoutEval {
        if p2_hp == 0 {
            if p1_hp == 0 {
                return PlayoutEval {
                    turns: 0,
                    u: Union { hp_rate: 1.0 },
                };
            } else {
                return Self::p1_wins(turn, p1_hp);
            }
        }
        if p1_hp == 0 {
            return Self::p2_wins(turn, p2_hp);
        }
        let hp_rate = (p2_max_hp * p1_hp) as f32 / (p1_max_hp * p2_hp) as f32;
        PlayoutEval {
            turns: 0,
            u: Union { hp_rate },
        }
    }

    pub fn both_dead() -> PlayoutEval {
        //評価値上はドローと同じ扱いだが、最後の戦いだと先に倒れたほうが負けになる。
        PlayoutEval {
            turns: 0,
            u: Union { hp_rate: 1.0 },
        }
    }
}
