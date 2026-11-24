use lotus_extra::{
    bb_modules,
    bb_system::{
        BBElectricPowerStreet, BBStarterMotor, BBVehicle, ElectricPowerStreet, StarterMotor,
        TickExtra,
        basic::BBSimple,
        doors::{
            BBDoors, Door, DoorRelease, DoorUnit, DoorUnitAutomatic, DoorUnitWing,
            DoorUnitWingLockMode, DoorUnitWingLockRelease, Doors, PneumaticBehaviour,
            StopBrakeController,
        },
        piston_traction::{BBPistonTractionTransfer, BBThrottleBrakeControl, ThrottleBrakeControl},
        road_vehicle::{
            Axle, BBAxle, BBRoadVehiclePneumatics, BBSteering, RoadVehiclePneumatics, Steering,
            SteeringProperties,
        },
    },
    vehicle::Rattling,
};
use lotus_script::{Animation, prelude::*};

use crate::{
    cockpit::{BBCockpitNd313, CockpitNd313},
    electric_power::{Nd313ElectricIndices, build_nd313_street},
    interface::Nd313Interface,
    lights::{BBLights, Nd313Lights},
    traction::{BBTraction, Traction},
};

mod cockpit;
mod electric_power;
mod interface;
mod lights;
mod traction;

const WHEEL_DIAMETER: f32 = 0.9;

pub(crate) const DOORS_MAX_SPEED_MPS: f32 = 3.0 / 3.6;
pub(crate) const MIN_THROTTLE_RELEASE_STOP_BRAKE: f32 = 0.1;

pub use electric_power::{BATTERY_VOLTAGE, NOMINAL_VOLTAGE};

struct Nd313Extras {
    rattling: Rattling,
}

impl Default for Nd313Extras {
    fn default() -> Self {
        Self {
            rattling: Rattling::builder()
                .animation(Animation::get("Main").unwrap())
                .sound_variable("snd_Rattling")
                .build(),
        }
    }
}

impl TickExtra for Nd313Extras {
    fn tick_extra(&mut self) {
        self.rattling.tick();
    }
}

type MyScript = BBVehicle<Modules, Backbone, Nd313Extras, Nd313Interface>;

pub struct Modules {
    axles: Vec<Axle>,
    pub(crate) electricity: ElectricPowerStreet<Nd313ElectricIndices>,
    pneumatics: RoadVehiclePneumatics,
    pub(crate) traction: Traction,
    throttle_brake_control: ThrottleBrakeControl,
    pub(crate) lights: Nd313Lights,
    pub(crate) starter: StarterMotor,
    pub(crate) doors: Doors,
    cockpit: CockpitNd313,
    steering: Steering,
}

impl Default for Modules {
    fn default() -> Self {
        let axles = vec![
            Axle::new(0, WHEEL_DIAMETER),
            Axle::new(1, WHEEL_DIAMETER)
                .with_tacho("DiffGear_mps".to_string())
                .with_traction(1, 5.74),
            Axle::new(2, WHEEL_DIAMETER),
        ];

        let pneumatics = RoadVehiclePneumatics::builder()
            .axles(axles.clone())
            .build();

        Self {
            steering: Steering::new(SteeringProperties::new(40.0)),
            electricity: build_nd313_street(),
            axles,
            pneumatics,
            throttle_brake_control: ThrottleBrakeControl::new(0, 1, 0.85),
            lights: Nd313Lights::default(),
            starter: StarterMotor::bus_24v(),
            cockpit: CockpitNd313::default(),
            doors: Doors::new()
                .add_release(DoorRelease)
                .add_stop_brake_controller(
                    StopBrakeController::default()
                        .add_check_all_doors_closed()
                        .add_check_throttle_pedal()
                        .add_check_vehicle_stopped()
                        .add_sound_deactivate("snd_StopBrake_Release"),
                )
                .add_door(DoorUnit::new(
                    vec![
                        DoorUnitWing::new(
                            Door::new_pneumatic(
                                2.5 / 698_700.0,
                                PneumaticBehaviour::Linear {
                                    rate: 0.5 / 698_700.0,
                                    p_end_normalized: 0.2,
                                },
                                (0.1, 0.1),
                            )
                            .with_friction(0.1)
                            .with_position_var("Door_1_1_Pos".to_string())
                            .with_sound_open("snd_Door11_Open")
                            .with_sound_close("snd_Door11_Close"),
                        )
                        .with_wing_lock_mode(DoorUnitWingLockMode::BlockOpen)
                        .with_wing_lock_release(DoorUnitWingLockRelease::SyncOpen),
                        DoorUnitWing::new(
                            Door::new_pneumatic(
                                2.4 / 698_700.0,
                                PneumaticBehaviour::Linear {
                                    rate: 0.4 / 698_700.0,
                                    p_end_normalized: 0.21,
                                },
                                (0.105, 0.09),
                            )
                            .with_friction(0.11)
                            .with_position_var("Door_1_2_Pos".to_string())
                            .with_sound_open("snd_Door12_Open")
                            .with_sound_close("snd_Door12_Close"),
                        )
                        .with_wing_lock_mode(DoorUnitWingLockMode::BlockOpen)
                        .with_wing_lock_release(DoorUnitWingLockRelease::SyncOpen),
                    ],
                    DoorUnitAutomatic::Manual,
                    Some(0),
                ))
                .add_door(DoorUnit::new(
                    vec![DoorUnitWing::new(
                        Door::new_pneumatic(
                            6.0 / 698_700.0,
                            PneumaticBehaviour::Linear {
                                rate: 0.36 / 698_700.0,
                                p_end_normalized: 0.3,
                            },
                            (0.01, 0.01),
                        )
                        .with_friction(0.2)
                        .with_position_var("Door_2_1_Pos".to_string())
                        .with_sound_open("snd_Door2_Open")
                        .with_sound_close("snd_Door2_Close")
                        .with_sound_close_bump("snd_Door2_Close_End"),
                    )],
                    DoorUnitAutomatic::Timer(5.5),
                    Some(0),
                ))
                .add_door(DoorUnit::new(
                    vec![DoorUnitWing::new(
                        Door::new_pneumatic(
                            6.0 / 698_700.0,
                            PneumaticBehaviour::Linear {
                                rate: 0.36 / 698_700.0,
                                p_end_normalized: 0.3,
                            },
                            (0.01, 0.01),
                        )
                        .with_friction(0.2)
                        .with_position_var("Door_3_1_Pos".to_string())
                        .with_sound_open("snd_Door3_Open")
                        .with_sound_close("snd_Door3_Close")
                        .with_sound_close_bump("snd_Door3_Close_End"),
                    )],
                    DoorUnitAutomatic::Timer(5.5),
                    Some(0),
                )),
            traction: Traction::default(),
        }
    }
}

bb_modules! {
    Modules => Backbone {
        tick {
            pneumatics => pneumatics;
            throttle_brake_control => throttle_brake_control;
            steering => steering;
            cockpit => cockpit;
            electricity => electricity;
            traction => traction;
            doors => doors;
            axles[1] => axle;
        }
        init {
            pneumatics => pneumatics;
            electricity => electricity;
            lights => lights;
            doors => doors;
            axles[1] => axle;
            cockpit => cockpit;
        }
        on_action {
            cockpit => cockpit;
        }
        on_message {
            traction => piston_traction_transfer;
            axles[1] => axle;
            cockpit => cockpit;
        }
        reset {
            cockpit, electricity, traction, throttle_brake_control, lights, starter, doors,
        }
    }
}

script!(MyScript);

#[derive(Default)]
pub struct Backbone {
    pub pneumatics: BBRoadVehiclePneumatics,
    pub throttle_brake_control: BBThrottleBrakeControl,
    pub steering: BBSteering,
    pub cockpit: BBCockpitNd313,
    pub electricity: BBElectricPowerStreet,
    pub traction: BBTraction,
    pub piston_traction_transfer: BBPistonTractionTransfer,
    pub lights: BBLights,
    pub starter: BBStarterMotor,
    pub doors: BBDoors,
    pub axle: BBAxle,
    pub retarder_request: BBSimple<i8>,
}
