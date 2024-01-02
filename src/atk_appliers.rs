/// この順番にApplyしていくので、順番も重要
pub enum AtkAppliers{
	わざわい器札,
	クオークチャージ攻,
	こだわりハチマキ等,
}

impl AtkAppliers{
	pub fn value(&self) -> usize{
		use AtkAppliers as A;
		match self{
			A::わざわい器札 => 3072,
			A::クオークチャージ攻 => 5325,
			A::こだわりハチマキ等 => 6144,
		}
	}
}