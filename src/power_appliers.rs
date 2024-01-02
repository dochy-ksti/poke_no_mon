use crate::pnum::PNum;

#[repr(u16)]
pub enum PowerAppliers {
    //てつのこぶし,
    //ちからずく,
    /// 弱いハチマキ、メガネも。 4505
    パンチグローブ等,
    /// 1.2 4915
    もくたん等,
}

impl PowerAppliers {
    pub fn value(&self) -> PNum {
        use PowerAppliers as A;
		use crate::pnum::PNum as P;
        match self {
            A::パンチグローブ等 => P::V1_1,
            A::もくたん等 => P::V1_2,
        }
    }

    pub fn from_u16(v: u16) -> PowerAppliers {
        unsafe { std::mem::transmute(v) }
    }
}
