// safety.rs - Hard limits the AI can never override. no_std friendly.

#[derive(Clone, Copy)]
pub struct Limits {
    pub max_conductivity_us: f32,
    pub min_water_level_pct: f32,
    pub max_return_temp_c: f32,
    pub min_biocide_ppm: f32,
    pub fan_min_pct: f32,
    pub fan_max_pct: f32,
}

#[derive(Clone, Copy)]
pub struct Readings {
    pub conductivity_us: f32,
    pub water_level_pct: f32,
    pub return_temp_c: f32,
    pub biocide_ppm: f32,
    pub sensors_ok: bool,
}

#[derive(Clone, Copy)]
pub struct Setpoints {
    pub bleed_open: bool,
    pub fan_pct: f32,
}

#[derive(Clone, Copy, Default)]
pub struct Alarms {
    pub sensor_fault: bool,
    pub high_conductivity: bool,
    pub low_water: bool,
    pub high_temp: bool,
    pub low_biocide: bool,
}

impl Alarms {
    pub fn any(&self) -> bool {
        self.sensor_fault || self.high_conductivity || self.low_water
            || self.high_temp || self.low_biocide
    }
}

pub fn enforce(ai: Setpoints, r: Readings, lim: Limits, fallback: Setpoints)
    -> (Setpoints, Alarms)
{
    let mut alarms = Alarms::default();

    if !r.sensors_ok {
        alarms.sensor_fault = true;
        return (fallback, alarms);
    }

    let mut out = ai;
    out.fan_pct = out.fan_pct.clamp(lim.fan_min_pct, lim.fan_max_pct);

    if r.conductivity_us > lim.max_conductivity_us {
        out.bleed_open = true;
        alarms.high_conductivity = true;
    }
    if r.water_level_pct < lim.min_water_level_pct {
        out.bleed_open = false; // low water wins
        alarms.low_water = true;
    }
    if r.return_temp_c > lim.max_return_temp_c {
        out.fan_pct = lim.fan_max_pct;
        alarms.high_temp = true;
    }
    if r.biocide_ppm < lim.min_biocide_ppm {
        alarms.low_biocide = true;
    }

    (out, alarms)
}
