use lotus_extra::bb_system::{
    DomainInterface, basic::BackBone, cockpit::TwoStepState, cockpit_enhanced::IgnitionSwitchStep,
};
use lotus_extra::messages::std::{BatteryvoltageSender, PowerSignalCabin, PowerSignalSender};

use crate::{Backbone, Modules};

#[derive(Default)]
pub struct Nd313PowerInterface {
    power_signal: PowerSignalSender,
    battery_voltage: BatteryvoltageSender,
}

impl DomainInterface<Modules, Backbone> for Nd313PowerInterface {
    fn wire(&mut self, modules: &Modules, backbone: &mut Backbone) {
        let idx = modules.electricity.indices();
        let ignition = backbone
            .cockpit
            .vdv_dashboard
            .ignition_switch
            .state()
            .get_state();
        let rpm = backbone.piston_traction_transfer.rpm;
        let fan = match backbone.cockpit.passender_fan.state().get_state() {
            TwoStepState::Off => 0.0,
            TwoStepState::StepA => 0.5,
            TwoStepState::StepB => 1.0,
        };
        let front_window = backbone.cockpit.front_window_heat.state().get_state();
        let mirror = backbone.cockpit.mirror_heat.state().get_state();
        let ignition_on = ignition >= IgnitionSwitchStep::Step1;

        backbone.electricity.set_control_circuit(0, true);
        backbone.electricity.set_control_circuit(1, ignition_on);
        backbone
            .electricity
            .set_control_circuit(2, ignition >= IgnitionSwitchStep::Step2);
        backbone
            .electricity
            .set_converter_relay(ignition >= IgnitionSwitchStep::Step2);
        backbone.electricity.set_engine_rpm(rpm);

        self.power_signal.send(ignition_on, PowerSignalCabin::A);
        self.battery_voltage.send(
            ignition_on,
            modules
                .electricity
                .unit_voltage_normalized_battery(&backbone.electricity, idx.fuse_dauerplus),
        );
        backbone
            .electricity
            .set_circuit_breaker(idx.fuse_dauerplus, true);
        backbone
            .electricity
            .set_circuit_breaker(idx.fuse_ignition, true);
        backbone.electricity.set_circuit_breaker(idx.fuse_run, true);

        modules
            .electricity
            .set_motor_input(&mut backbone.electricity, idx.fan, fan);
        modules
            .electricity
            .set_motor_running(&mut backbone.electricity, idx.fan, fan > 0.0);

        modules.electricity.set_heater_input_bool(
            &mut backbone.electricity,
            idx.heater_front_window,
            front_window,
        );
        modules.electricity.set_heater_input_bool(
            &mut backbone.electricity,
            idx.heater_mirror,
            mirror,
        );
    }
}
