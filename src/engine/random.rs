use macroquad_toolkit::rng as game_rng;

pub fn next_u64() -> u64 {
    let high = game_rng::gen_range(0u32, u32::MAX) as u64;
    let low = game_rng::gen_range(0u32, u32::MAX) as u64;
    (high << 32) | low
}
