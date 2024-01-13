use crate::pnum::PNum;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Items{
	なし,
	/// 威力補正1.1
	パンチグローブ等,
	/// 威力補正1.2
	もくたん等,

	/// ダメージ補正1.3 5325
	いのちのたま等,

	/// 攻撃補正0.75 3072
	わざわい器札,

	/// 攻撃補正1.5 6144
	こだわりハチマキ等,

	/// 防御補正1.5 6144
	とつげきチョッキ,

	
	ブーストエナジー,

	こだわりスカーフ
}

impl Items{
	pub fn speed_modifier(&self) -> Option<PNum>{
		use Items as I;
		use PNum as P;
		match self{
			I::こだわりスカーフ => Some(P::V1_5),
			_ => None,
		}
	}
}