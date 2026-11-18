// safety.rs — FrostGuard hard limits. The AI can never override these.
use std::time::{Duration, Instant};

#[derive(Clone, Debug)]
pub struct Limits {
    pub max_conductivity_us: f32,   // scaling risk ceiling (set per site)
    pub min_water_level_pct: f32,
    pub max_water_temp_c: f32,      // Legionella risk window guard
    pub fan_min_pct: f32,
    pub fan_max_pct: f32,
    pub bleed_max_pct: f32,
    pub sensor_timeout: Duration,
}

#[derive(Clone, Debug)]
pub struct Readings {
    pub conductivity_us: f32,
    pub water_level_pct: f32,
    pub water_temp_c: f32,
    pub last_update: Instant,
}

#[derive(Clone, Debug)]
pub struct Setpoints {
    pub fan_pct: f32,
    pub bleed_pct: f32,
}

pub enum Decision {
    Approved(Setpoints),          // AI suggestion within limits
    Clamped(Setpoints, String),   // adjusted to fit limits
    Fallback(Setpoints, String),  // unsafe: use site defaults
}

pub fn check(ai: &Setpoints, r: &Readings, lim: &Limits, defaults: &Setpoints) -> Decision {
    // 1. Stale or missing sensor data -> fall back.
    if r.last_update.elapsed() > lim.sensor_timeout {
        return Decision::Fallback(defaults.clone(), "sensor data stale".into());
    }
    // 2. Dangerous conditions -> fall back.
    if r.water_level_pct < lim.min_water_level_pct {
        return Decision::Fallback(defaults.
