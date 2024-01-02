pub enum PowerAppliers{
	//てつのこぶし,
	//ちからずく,
	/// 弱いハチマキ、メガネも。 4505
	パンチグローブ等,
	/// 1.2 4915
	もくたん等,
}

impl PowerAppliers{
	pub fn value(&self) -> usize{
		use PowerAppliers as P;
		match self{
			P::パンチグローブ等 => 4505,
			P::もくたん等 => 4915,
		}
	}
}