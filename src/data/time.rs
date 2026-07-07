//! Player-facing time helpers.
//!
//! Internally the simulation advances in ticks (the main sim step runs every
//! [`TICKS_PER_DAY`] ticks, and a season spans `TICKS_PER_DAY * 60`). The
//! fiction, however, is measured in days — the log and mission cards must never
//! leak raw engine "ticks" to the player.

/// Simulation ticks that make up one in-world day.
pub const TICKS_PER_DAY: u32 = 60;

/// Convert a tick span to whole in-world days, rounded up so any nonzero span
/// reads as at least "1 day" rather than "0 days".
pub fn ticks_to_days(ticks: u32) -> u32 {
    if ticks == 0 {
        0
    } else {
        ticks.div_ceil(TICKS_PER_DAY)
    }
}

/// Format a tick span as a player-facing day label, e.g. `"1 day"` / `"30 days"`.
pub fn days_label(ticks: u32) -> String {
    match ticks_to_days(ticks) {
        0 => "less than a day".to_string(),
        1 => "1 day".to_string(),
        days => format!("{} days", days),
    }
}
