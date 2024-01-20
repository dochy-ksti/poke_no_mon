#![allow(dead_code, unused_variables, unused_imports)]

pub mod ability_items;
pub mod appliers;
pub mod moves;
pub mod pnum;
pub mod poke_params;
pub mod simulations;
pub mod environment;
// 参考資料
// PokeWiki ターン
// https://wiki.xn--rckteqa2e.com/wiki/%E3%82%BF%E3%83%BC%E3%83%B3
// ↑ クオークチャージ、こだいかっせいに関して古い処理になっている　詳細は PokeWiki クオークチャージを見るべし
// https://wiki.xn--rckteqa2e.com/wiki/%E3%82%AF%E3%82%A9%E3%83%BC%E3%82%AF%E3%83%81%E3%83%A3%E3%83%BC%E3%82%B8
// PokeWiki ダメージ
// https://wiki.xn--rckteqa2e.com/wiki/%E3%83%80%E3%83%A1%E3%83%BC%E3%82%B8

const PLAYOUT_TURNS : usize = 3;

fn main() {
    //let s = damage_move_storage();
    //println!("{:#?}", s);
}
