use lotus_extra::bb_system::{
    DomainInterface,
    basic::{BackBone, ModuleTickInput},
    cockpit::TwoStepState,
    cockpit_enhanced::IgnitionSwitchStep,
    electric_appliances::{INCANDESCENT_LUMINOUS_FLUX_EXPONENT, incandescent_brightness},
    lights::IndicatorState,
};
use lotus_extra::math::IfElse;
use lotus_extra::messages::std::LightSender;
use lotus_script::var::set_var;

use crate::lights::Nd313LightsInput;
use crate::{Backbone, Modules};

#[derive(Default)]
pub struct Nd313LightsInterface {
    light: LightSender,
}

impl DomainInterface<Modules, Backbone> for Nd313LightsInterface {
    fn wire(&mut self, modules: &Modules, backbone: &mut Backbone) {
        let idx = modules.electricity.indices();
        let lamps = idx.lamps;
        let electric = &modules.electricity;
        let rpm = backbone.piston_traction_transfer.rpm;
        let bb_electric = &mut backbone.electricity;
        let bb_cockpit = &backbone.cockpit.vdv_dashboard;

        let ignition = bb_cockpit.ignition_switch.state().get_state();
        let step2 = ignition >= IgnitionSwitchStep::Step2;
        let u_dauerplus = electric.unit_voltage_normalized(bb_electric, idx.fuse_dauerplus);
        let u_run = electric.unit_voltage_normalized(bb_electric, idx.fuse_run);

        let inside_step = bb_cockpit.inside_light_step_switch.state().get_state();
        electric.set_lamp_input_bool(
            bb_electric,
            lamps.inside_lower_deck,
            inside_step != TwoStepState::Off,
        );
        electric.set_lamp_input_bool(
            bb_electric,
            lamps.inside_lower_front_right,
            inside_step == TwoStepState::StepB,
        );
        electric.set_lamp_input_bool(
            bb_electric,
            lamps.inside_upper_deck,
            bb_cockpit.inside_light_secondary_switch.state().get_state(),
        );
        electric.set_lamp_input_bool(
            bb_electric,
            lamps.inside_driver,
            bb_cockpit.inside_light_driver_switch.state().get_state(),
        );

        let park = bb_cockpit.modern_outside_light_switch.parking();
        let dim = bb_cockpit.modern_outside_light_switch.dim();
        let brake = backbone.throttle_brake_control.brake_value() > 0.02;

        self.light.send(if park { u_dauerplus } else { 0.0 });

        electric.set_lamp_input_bool(bb_electric, lamps.park, park);
        electric.set_lamp_input_bool(bb_electric, lamps.park_led, park);
        electric.set_lamp_input_bool(bb_electric, lamps.dim, dim);
        electric.set_lamp_input_bool(bb_electric, lamps.brake, brake);
        electric.set_lamp_input_bool(bb_electric, lamps.brake_led, brake);

        let indicator = if u_dauerplus > 0.0 && bb_cockpit.flash_light_switch.state().get_state() {
            IndicatorState::Warning
        } else if !step2 {
            IndicatorState::Off
        } else {
            bb_cockpit.indicator_switch.state().get_state().into()
        };

        let indicator_voltage = if indicator == IndicatorState::Warning {
            u_dauerplus
        } else {
            u_run
        };

        let bright = incandescent_brightness(u_dauerplus, INCANDESCENT_LUMINOUS_FLUX_EXPONENT);
        let park_f: f32 = park.if_else(1.0, 0.0);
        let dim_f: f32 = dim.if_else(1.0, 0.0);
        let brake_f: f32 = brake.if_else(1.0, 0.0);

        modules.lights.tick(
            &mut backbone.lights,
            Nd313LightsInput {
                indicator,
                indicator_voltage_normalized: indicator_voltage,
                dim_scale: dim_f.max(park_f * 0.4) * bright,
                dim_blue: dim_f.max(park_f * 0.5) * bright,
                rear_n_brake: brake_f.max(park_f * 0.7) * bright,
            },
        );

        let relay_on = backbone.lights.indicator.relay_state();
        let left_on = indicator == IndicatorState::Left || indicator == IndicatorState::Warning;
        let right_on = indicator == IndicatorState::Right || indicator == IndicatorState::Warning;
        electric.set_lamp_input_bool(bb_electric, lamps.blinker_left, left_on && relay_on);
        electric.set_lamp_input_bool(bb_electric, lamps.blinker_left_led, left_on && relay_on);
        electric.set_lamp_input_bool(bb_electric, lamps.blinker_right, right_on && relay_on);
        electric.set_lamp_input_bool(bb_electric, lamps.blinker_right_led, right_on && relay_on);

        set_var("debug_rpm", rpm);
        set_var(
            "debug_voltage",
            bb_electric.unit_voltage_v(electric.main_bus_index()),
        );
    }
}
