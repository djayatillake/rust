// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]



pub fn production_rate_per_hour(speed: u8) -> f64 {
    let unadjusted_rate = (221 * speed as u32) as f64;
    if speed < 5 {
        unadjusted_rate
    } else if speed < 9 {
        unadjusted_rate * 0.9
    } else {
        unadjusted_rate * 0.77
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    production_rate_per_hour(speed) as u32 / 60
}
