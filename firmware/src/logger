// logger.rs - Tracks bleed water and fan energy for savings reports. no_std.

pub struct Logger {
    bleed_litres_per_min: f32,
    fan_watts_full: f32,
    bleed_seconds: u32,
    fan_watt_seconds: f32,
    pub uptime_seconds: u32,
}

impl Logger {
    pub const fn new(bleed_litres_per_min: f32, fan_watts_full: f32) -> Self {
        Self {
            bleed_litres_per_min,
            fan_watts_full,
            bleed_seconds: 0,
            fan_watt_seconds: 0.0,
            uptime_seconds: 0,
        }
    }

    pub fn tick(&mut self, bleed_open: bool, fan_pct: f32, secs: u32) {
        if bleed_open {
            self.bleed_seconds += secs;
        }
        self.fan_watt_seconds += self.fan_watts_full * (fan_pct / 100.0) * secs as f32;
        self.uptime_seconds += secs;
    }

    pub fn bleed_litres(&self) -> f32 {
        self.bleed_seconds as f32 / 60.0 * self.bleed_litres_per_min
    }

    pub fn fan_wh(&self) -> f32 {
        self.fan_watt_seconds / 3600.0
    }
}
