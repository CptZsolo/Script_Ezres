use lotus_extra::{
    bb_system::{
        DomainInterface, StarterMotorInput,
        basic::{BackBone, ModuleTickInput},
        cockpit_enhanced::IgnitionSwitchStep,
    },
    math::IfElse,
    messages::std::{EngineStartStop, RetarderRequest},
};

use crate::{Backbone, Modules};

#[derive(Default)]
pub struct Nd313TractionInterface;

impl DomainInterface<Modules, Backbone> for Nd313TractionInterface {
    fn wire(&mut self, modules: &Modules, backbone: &mut Backbone) {
        let idx = modules.electricity.indices();
        let ignition = backbone
            .cockpit
            .vdv_dashboard
            .ignition_switch
            .state()
            .get_state();
        let rpm = backbone.piston_traction_transfer.rpm;
        let bus_v = backbone
            .electricity
            .unit_voltage_v(modules.electricity.main_bus_index());

        modules.starter.tick(
            &mut backbone.starter,
            StarterMotorInput {
                enable: ignition == IgnitionSwitchStep::Starter,
                bus_voltage_v: bus_v,
                engine_rpm: rpm,
            },
        );

        backbone.electricity.add_load_conductance(
            modules.electricity.main_bus_index(),
            idx.load_starter,
            modules.starter.conductance_s(&backbone.starter),
        );

        let start_stop = if ignition <= IgnitionSwitchStep::Step1 {
            EngineStartStop::Stop
        } else if modules.starter.engaged(&backbone.starter) {
            EngineStartStop::Start
        } else {
            EngineStartStop::None
        };

        modules
            .traction
            .piston
            .starter_relay(&mut backbone.traction.piston_traction, start_stop);

        backbone
            .cockpit
            .vdv_dashboard
            .automatic_gear_box_mode_switch_group
            .state()
            .call_on_changed(|state| {
                modules.traction.piston.send_gearbox_mode(&state);
            });

        let retarder_request = (backbone.throttle_brake_control.brake_value() > 0.02).if_else(1, 0);

        if backbone.retarder_request.set_if_different(retarder_request) {
            modules
                .traction
                .piston
                .send_retarder_request(&RetarderRequest(retarder_request));
        }
    }
}
