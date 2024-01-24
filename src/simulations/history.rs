
/// 本当はLinkedListAllocatorを使ってアロック&フリーを低コストにしながら
/// SinglyLinkedListで一つずつHistoryを付け加えていく実装にすれば深さ優先探索でのヒストリーの記録には
/// 深さ*2のメモリしか必要ないはずなんだけど、いいライブラリがみつからない
/// Effective C++にアロケータの作り方は書いてあったんだが...RustではAllocatorを書くのは一般的でないようだ。
/// 毎回LinkedListNodeをallocしても全然問題ないとは思うが...32バイトのヒストリー記録構造体を返すことにする。
#[derive(Debug, Default)]
pub struct History {
    // 選択肢の数は256個まで、ということになる。
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

    pub fn push(&mut self, val: u8) {
        let len = self.len();
        if LEN_INDEX <= len {
            panic!("History: Array Index Out of Bound")
        }
        self.array[len] = val;
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
