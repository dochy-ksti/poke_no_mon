/// ポケモンで使われている fixed point number。4096を分母にしている。
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct PNum {
    pub val: u32,
}

impl PNum {
    const fn new(val: u32) -> PNum {
        PNum { val }
    }
    pub const V0: PNum = PNum::new(0);
    pub const V0_25: PNum = PNum::new(1024);
    pub const V0_5: PNum = PNum::new(2048);
    pub const V0_75: PNum = PNum::new(3072);
    pub const V1: PNum = PNum::new(4096);
    pub const V1_1: PNum = PNum::new(4506);
    pub const V1_2: PNum = PNum::new(4915);
    pub const V1_3: PNum = PNum::new(5325);
    pub const V1_5: PNum = PNum::new(6144);
    pub const V2: PNum = PNum::new(8192);

    /// 四捨五入
    pub fn apply4(&self, r: PNum) -> PNum {
        PNum::new((self.val * r.val + 2048) >> 12)
    }
    /// 五捨五超入
    pub fn apply5(&self, r: u32) -> u32 {
        (self.val * r + 2047) >> 12
    }

    /// 切り捨て
    pub fn apply(&self, r: u32) -> u32 {
        (self.val * r) >> 12
    }

    /// 切り捨て
    pub fn mul(&self, r: PNum) -> PNum {
        PNum::new((self.val * r.val) >> 12)
    }
}
