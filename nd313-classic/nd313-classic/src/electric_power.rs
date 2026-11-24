//! ND313-Bordnetz: [`ElectricPowerStreet`], Sicherungen und Geräteindizes.

use lotus_extra::bb_system::{
    ElectricBatteryModel, ElectricPowerStreet, charging_voltage_from_nominal,
};

pub const BATTERY_VOLTAGE: f32 = 24.0;
pub const NOMINAL_VOLTAGE: f32 = charging_voltage_from_nominal(BATTERY_VOLTAGE);

pub const LAMP_FADE_IN: f32 = 8.0;
pub const LAMP_FADE_OUT: f32 = 8.0;

/// R = U²/P bei 24 V.
const R24: f32 = BATTERY_VOLTAGE * BATTERY_VOLTAGE;

pub const LAMP_R_INTERIOR: f32 = R24 / 40.0;
pub const LAMP_R_DRIVER: f32 = R24 / 21.0;
pub const LAMP_R_PARK: f32 = R24 / 10.0;
pub const LAMP_R_DIM: f32 = R24 / 110.0;
pub const LAMP_R_BRAKE: f32 = R24 / 42.0;
pub const LAMP_R_BLINKER: f32 = R24 / 21.0;
pub const LAMP_R_LED: f32 = R24 / 5.0;

pub const MOTOR_R_FAN_RUNNING: f32 = 8.0;
pub const MOTOR_R_FAN_STALL: f32 = 2.0;
pub const HEATER_R_FRONT_WINDOW: f32 = 2.4;
pub const HEATER_R_MIRROR: f32 = 9.6;

#[derive(Clone, Copy, Debug, Default)]
pub struct Nd313LampIndices {
    pub inside_lower_deck: usize,
    pub inside_upper_deck: usize,
    pub inside_lower_front_right: usize,
    pub inside_driver: usize,
    pub park: usize,
    pub park_led: usize,
    pub dim: usize,
    pub brake: usize,
    pub brake_led: usize,
    pub blinker_left: usize,
    pub blinker_left_led: usize,
    pub blinker_right: usize,
    pub blinker_right_led: usize,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Nd313ElectricIndices {
    pub fuse_dauerplus: usize,
    pub fuse_ignition: usize,
    pub fuse_run: usize,
    pub load_starter: usize,
    pub fan: usize,
    pub heater_front_window: usize,
    pub heater_mirror: usize,
    pub lamps: Nd313LampIndices,
}

fn add_lamp(
    electricity: &mut ElectricPowerStreet<Nd313ElectricIndices>,
    bus: usize,
    resistance_ohm: f32,
    variable: Option<&str>,
) -> usize {
    let idx = electricity.add_lamp_get_index(
        bus,
        resistance_ohm,
        LAMP_FADE_IN,
        LAMP_FADE_OUT,
        NOMINAL_VOLTAGE,
    );
    if let Some(name) = variable {
        electricity.set_appliance_variable(idx, name);
    }
    idx
}

fn add_led(
    electricity: &mut ElectricPowerStreet<Nd313ElectricIndices>,
    bus: usize,
    variable: Option<&str>,
) -> usize {
    let idx = electricity.add_led_get_index(bus, LAMP_R_LED, NOMINAL_VOLTAGE);
    if let Some(name) = variable {
        electricity.set_appliance_variable(idx, name);
    }
    idx
}

/// LiMa voll ab dieser Motordrehzahl. Default im Netz ist 1500 — im Diesel-Leerlauf
/// (ca. 600) läge die Klemmenspannung dann unter der Batterie, Ladung erst bei Vollgas.
const ALTERNATOR_FULL_POWER_RPM: f32 = 650.0;

/// Zündstufen-Unterspannung. Unter dem Anlass-Einbruch (~18 V), damit nichts ausgeht.
const CONTROL_MIN_VOLTAGE_V: f32 = 15.0;

/// Innenwiderstand für ~18 V am 500-A-Anlasser (Default 0,05 Ω sackt auf ~10 V).
const BATTERY_INTERNAL_RESISTANCE_OHM: f32 = 0.008;

pub fn build_nd313_street() -> ElectricPowerStreet<Nd313ElectricIndices> {
    let mut electricity = ElectricPowerStreet::new(
        BATTERY_VOLTAGE,
        &[0.0, CONTROL_MIN_VOLTAGE_V, CONTROL_MIN_VOLTAGE_V],
    )
    .with_battery(
        ElectricBatteryModel::new(BATTERY_VOLTAGE)
            .with_internal_resistance_ohm(BATTERY_INTERNAL_RESISTANCE_OHM),
    )
    .with_alternator_full_power_rpm(ALTERNATOR_FULL_POWER_RPM);

    let fuse_dauerplus = electricity.add_circuit_breaker_get_index(0);
    let fuse_ignition = electricity.add_circuit_breaker_get_index(1);
    let fuse_run = electricity.add_circuit_breaker_get_index(2);
    let load_starter = electricity.add_load_get_index(electricity.main_bus_index());

    let fan = electricity.add_motor_get_index(
        fuse_ignition,
        MOTOR_R_FAN_RUNNING,
        MOTOR_R_FAN_STALL,
        NOMINAL_VOLTAGE,
    );
    let heater_front_window =
        electricity.add_heater_get_index(fuse_run, HEATER_R_FRONT_WINDOW, NOMINAL_VOLTAGE);
    let heater_mirror =
        electricity.add_heater_get_index(fuse_ignition, HEATER_R_MIRROR, NOMINAL_VOLTAGE);

    let lamps = Nd313LampIndices {
        inside_lower_deck: add_lamp(
            &mut electricity,
            fuse_dauerplus,
            LAMP_R_INTERIOR,
            Some("Light_Cabin_OtherLights"),
        ),
        inside_upper_deck: add_lamp(
            &mut electricity,
            fuse_dauerplus,
            LAMP_R_INTERIOR,
            Some("Light_Cabin_Upper"),
        ),
        inside_lower_front_right: add_lamp(
            &mut electricity,
            fuse_dauerplus,
            LAMP_R_INTERIOR,
            Some("Light_Cabin_FirstLight"),
        ),
        inside_driver: add_lamp(
            &mut electricity,
            fuse_dauerplus,
            LAMP_R_DRIVER,
            Some("Light_Cabin_Driver"),
        ),
        park: add_lamp(
            &mut electricity,
            fuse_dauerplus,
            LAMP_R_PARK,
            Some("Light_ParkingNRear"),
        ),
        park_led: add_led(
            &mut electricity,
            fuse_dauerplus,
            Some("Light_ParkingNRear_LED"),
        ),
        dim: add_lamp(
            &mut electricity,
            fuse_ignition,
            LAMP_R_DIM,
            Some("Light_DimLight"),
        ),
        brake: add_lamp(
            &mut electricity,
            fuse_dauerplus,
            LAMP_R_BRAKE,
            Some("Light_Brake"),
        ),
        brake_led: add_led(&mut electricity, fuse_dauerplus, Some("Light_Brake_LED")),
        blinker_left: add_lamp(&mut electricity, fuse_dauerplus, LAMP_R_BLINKER, None),
        blinker_left_led: add_led(&mut electricity, fuse_dauerplus, None),
        blinker_right: add_lamp(&mut electricity, fuse_dauerplus, LAMP_R_BLINKER, None),
        blinker_right_led: add_led(&mut electricity, fuse_dauerplus, None),
    };

    electricity.set_indices(Nd313ElectricIndices {
        fuse_dauerplus,
        fuse_ignition,
        fuse_run,
        load_starter,
        fan,
        heater_front_window,
        heater_mirror,
        lamps,
    });

    electricity
}
