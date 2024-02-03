
/// 禁断のpremature optimization
#[derive(Debug, Default)]
pub struct History {
    // 選択肢の数は256個まで、ということになる。本来は技4つ+交換2つで6手しかない。
    array: [u8; ARRAY_LEN],
}

const LEN_INDEX: usize = 31;
///16だと結構使い切れるが、32だと現実的な時間で全探索するのは無理だと思う。しらんけど
const ARRAY_LEN: usize = 32;

impl History {
    pub fn new() -> History {
        Default::default()
    }

    fn len(&self) -> usize {
        self.array[LEN_INDEX] as usize
    }

    pub fn push(&mut self, val: usize) {
        let len = self.len();
        if LEN_INDEX <= len {
            panic!("History: Array Index Out of Bound")
        }
        self.array[len] = val as u8;
        self.array[LEN_INDEX] += 1;
    }

    pub fn to_vec(&self) -> Vec<usize> {
        let mut vec: Vec<usize> = vec![];
        for i in 0..self.len() {
            vec.push(self.array[i] as usize)
        }
        vec
    }
}
