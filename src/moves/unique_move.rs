use strum::EnumCount;
use strum_macros::{EnumCount as EnumCountMacro, EnumString};

#[repr(usize)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, EnumString, EnumCountMacro)]
pub enum UniqueMove {
	NotUnique,
    // アイテムがないと威力が倍に
    アクロバット,
	// 特殊技だが相手の防御で計算
	サイコショック,
    // 半分を回復する
    //ドレインパンチ,
    // 体力を半分にする
    //カタストロフィ,
    // 先行で打つと飛行タイプがなくなる
    //はねやすめ,
    // 天気によってタイプが変わる
    //ウェザーボール,
}

impl UniqueMove {}
