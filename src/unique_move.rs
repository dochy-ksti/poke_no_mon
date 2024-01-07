
use strum::EnumCount;
use strum_macros::{EnumString, EnumCount as EnumCountMacro};

use crate::{types::Types, pnum::PNum};


#[repr(usize)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, EnumString, EnumCountMacro)]
pub enum PokeMove {


    // アイテムがないと威力が倍に
    アクロバット,
	// 半分を回復する
	//ドレインパンチ,
	// 体力を半分にする
	//カタストロフィ,
	// 先行で打つと飛行タイプがなくなる
    //はねやすめ,
    // 天気によってタイプが変わる
    //ウェザーボール,
}


impl PokeMove{
	
}

