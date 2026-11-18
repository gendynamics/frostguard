// FrostGuard - Cooling tower water optimizer
// ESP32-S3 | no_std | AI suggests, safety.rs decides.

#![no_std]
#![no_main]

mod safety;

use core::panic::PanicInfo;
use esp_hal::delay::Delay;
use safety::{enforce, Limits, Readings, Setpoints};

esp_bootloader_esp_idf::esp_app_desc!();

const LIMITS: Limits = Limits {
    max_conductivity_us: 3000.0,
    min_water_level_pct: 40.0,
    max_return_temp_c: 35.0,
    min_biocide_ppm: 0.5,
    fan_min_pct: 20.0,
    fan_max_pct: 100.0,
};

// The site's normal settings, used if sensors fail.
const FALLBACK: Setpoints = Setpoints { bleed_open: false, fan_pct: 70.0 };

// Target conductivity: bleed off only when needed, which saves water.
const TARGET_CONDUCTIVITY_US: f32 = 2500.0;

#[esp_hal::main]
fn main() -> ! {
    let _peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();

    loop {
        let r = read_sensors();
        let ai = ai_suggest(&r);
        let (safe, alarms) = enforce(ai, r, LIMITS, FALLBACK);

        set_bleed_valve(safe.bleed_open);
        set_fan_speed(safe.fan_pct);
        if alarms.any() {
            raise_alarm();
        }

        delay.delay_millis(1000);
    }
}

// Simple first "AI": hold conductivity near target,
// and slow the fans when the water is already cool.
fn ai_suggest(r: &Readings) -> Setpoints {
    let bleed_open = r.conductivity_us > TARGET_CONDUCTIVITY_US;
    let fan_pct = if r.return_temp_c < 28.0 { 40.0 } else { 80.0 };
    Setpoints { bleed_open, fan_pct }
}

// Placeholders: swap in real sensor reads.
fn read_sensors() -> Readings {
    Readings {
        conductivity_us: 2400.0,
        water_level_pct: 75.0,
        return_temp_c: 30.0,
        biocide_ppm: 1.0,
        sensors_ok: true,
    }
}

fn set_bleed_valve(_open: bool) {}
fn set_fan_speed(_pct: f32) {}
fn raise_alarm() {}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
