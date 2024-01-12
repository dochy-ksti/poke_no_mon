pub fn calc_rank(value: u32, rank: i32) -> u32 {
    if 0 <= rank {
        (value * (2 + rank as u32)) / 2
    } else {
        (value * 2) / (2 + (-rank) as u32)
    }
}
