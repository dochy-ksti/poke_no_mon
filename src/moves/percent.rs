#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Percent {
    pub val: u32,
}

impl Percent {
    pub fn new(val: u32) -> Self {
        Self { val }
    }

    pub const V0: Percent = Percent { val: 0 };

    /// 四捨五入。0の場合0、1の場合1。
    pub fn calc4(&self, r: u32) -> u32 {
        if r == 0 {
            0
        } else if r == 1 {
            1
        } else {
            r * (self.val * 10 + 5) / 1000
        }
    }
}
