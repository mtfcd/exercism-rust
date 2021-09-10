// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

fn success_rate(speed: u8) -> f64 {
    match speed {
        n if n <= 4 => 1.0,
        n if n <= 8 => 0.9,
        _ => 0.77
    }
}

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let total = speed as f64 * 221.;
    total * success_rate(speed)
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let total = speed as f64 * 221. * success_rate(speed) / 60.;
    total.floor() as u32
}
